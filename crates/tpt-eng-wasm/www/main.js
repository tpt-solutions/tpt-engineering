// Browser glue for the three demos. Every number shown comes from the WASM
// build in ./pkg, and all text is written with textContent, not innerHTML.
import init, {
  steam_state,
  rectangle_section,
  circle_section,
  i_section,
  pid_step_response,
} from "./pkg/tpt_eng_wasm.js";

const $ = (id) => document.getElementById(id);
const MM = 1e-3;

function row(label, value) {
  const tr = document.createElement("tr");
  const th = document.createElement("th");
  th.textContent = label;
  const td = document.createElement("td");
  td.className = "num";
  td.textContent = value;
  tr.append(th, td);
  return tr;
}

function table(rows) {
  const t = document.createElement("table");
  t.append(...rows);
  return t;
}

function fmt(x, digits = 4) {
  return Number.isFinite(x) ? x.toPrecision(digits) : "–";
}

// ---------------------------------------------------------------------------
// Steam tables
// ---------------------------------------------------------------------------

function renderSteam() {
  const out = $("steam-out");
  out.replaceChildren();
  out.className = "";
  const t = Number($("steam-t").value) + 273.15;
  const p = Number($("steam-p").value) * 1000;
  try {
    const s = steam_state(t, p);
    const badge = document.createElement("span");
    badge.className = s.in_range ? "badge ok" : "badge warn";
    badge.textContent = s.in_range ? "within IF97 range" : "outside IF97 range";
    const rows = [
      row("Region", s.region),
      row("Specific volume (m³/kg)", fmt(s.specific_volume_m3_per_kg)),
      row("Density (kg/m³)", fmt(s.density_kg_per_m3)),
      row("Enthalpy (kJ/kg)", fmt(s.enthalpy_kj_per_kg)),
      row("Entropy (kJ/(kg·K))", fmt(s.entropy_kj_per_kg_k)),
      row("cp (kJ/(kg·K))", fmt(s.isobaric_heat_capacity_kj_per_kg_k)),
    ];
    out.append(badge, table(rows));
  } catch (err) {
    out.textContent = String(err.message ?? err);
    out.className = "error";
  }
}

// ---------------------------------------------------------------------------
// Section properties
// ---------------------------------------------------------------------------

function renderSection() {
  const shape = $("sec-shape").value;
  for (const el of document.querySelectorAll("[data-i-only]")) {
    el.hidden = shape !== "i";
  }
  const w = Number($("sec-w").value) * MM;
  const h = Number($("sec-h").value) * MM;
  const out = $("section-out");
  out.replaceChildren();
  out.className = "";
  try {
    let s;
    if (shape === "rectangle") {
      s = rectangle_section(w, h);
    } else if (shape === "circle") {
      s = circle_section(w);
    } else {
      s = i_section(
        h,
        w,
        Number($("sec-tf").value) * MM,
        Number($("sec-tw").value) * MM,
      );
    }
    // SI outputs: m², m⁴, m³. Shown in mm-based units for readability.
    out.append(
      table([
        row("Area (mm²)", fmt(s.area * 1e6)),
        row("I_x (mm⁴)", fmt(s.second_moment_x * 1e12)),
        row("I_y (mm⁴)", fmt(s.second_moment_y * 1e12)),
        row("S_x (mm³)", fmt(s.section_modulus_x * 1e9)),
        row("S_y (mm³)", fmt(s.section_modulus_y * 1e9)),
        row("Z_x plastic (mm³)", fmt(s.plastic_modulus_x * 1e9)),
        row("Torsion constant J (mm⁴)", fmt(s.torsional_constant * 1e12)),
      ]),
    );
  } catch (err) {
    out.textContent = String(err.message ?? err);
    out.className = "error";
  }
}

// ---------------------------------------------------------------------------
// PID tuner
// ---------------------------------------------------------------------------

function drawPlot(response, setpoint, dt) {
  const canvas = $("pid-plot");
  const ctx = canvas.getContext("2d");
  const { width: W, height: H } = canvas;
  const styles = getComputedStyle(canvas);
  ctx.clearRect(0, 0, W, H);
  ctx.fillStyle = styles.getPropertyValue("--panel").trim() || "#fff";
  ctx.fillRect(0, 0, W, H);

  const tEnd = (response.length - 1) * dt;
  const yMax = Math.max(setpoint, ...response) * 1.1 || 1;
  const yMin = Math.min(0, ...response) * 1.1;
  const px = (t) => 40 + (t / tEnd) * (W - 56);
  const py = (y) => H - 24 - ((y - yMin) / (yMax - yMin)) * (H - 40);

  ctx.strokeStyle = styles.getPropertyValue("--border").trim() || "#ccc";
  ctx.beginPath();
  ctx.moveTo(40, py(setpoint));
  ctx.lineTo(W - 16, py(setpoint));
  ctx.setLineDash([6, 4]);
  ctx.stroke();
  ctx.setLineDash([]);

  ctx.strokeStyle = styles.getPropertyValue("--accent").trim() || "#2a6fdb";
  ctx.lineWidth = 2;
  ctx.beginPath();
  response.forEach((y, i) => {
    const x = px(i * dt);
    if (i === 0) ctx.moveTo(x, py(y));
    else ctx.lineTo(x, py(y));
  });
  ctx.stroke();

  ctx.fillStyle = styles.getPropertyValue("--muted").trim() || "#666";
  ctx.font = "12px system-ui, sans-serif";
  ctx.fillText(`t = 0 … ${tEnd.toFixed(1)} s`, 40, H - 6);
  ctx.fillText(`setpoint ${setpoint}`, W - 130, py(setpoint) - 6);
}

function renderPid() {
  const kp = Number($("pid-kp").value);
  const ki = Number($("pid-ki").value);
  const kd = Number($("pid-kd").value);
  $("pid-kp-v").textContent = kp.toFixed(2);
  $("pid-ki-v").textContent = ki.toFixed(2);
  $("pid-kd-v").textContent = kd.toFixed(2);

  const tau = Number($("pid-tau").value);
  const setpoint = Number($("pid-sp").value);
  const limitText = $("pid-lim").value.trim();
  const limit = limitText === "" ? undefined : Number(limitText);
  // Sample at 1/200 of the time constant, and simulate for 10 time constants.
  const dt = tau / 200;
  const duration = 10 * tau;
  try {
    const response = pid_step_response(kp, ki, kd, setpoint, tau, dt, duration, limit);
    drawPlot(response, setpoint, dt);
    const last = response[response.length - 1];
    const peak = Math.max(...response);
    $("pid-out").textContent =
      `final value ${fmt(last)}, peak ${fmt(peak)}, ` +
      `overshoot ${fmt(Math.max(0, (peak - setpoint) / setpoint) * 100, 3)} %`;
    $("pid-error").textContent = "";
  } catch (err) {
    $("pid-error").textContent = String(err.message ?? err);
  }
}

// ---------------------------------------------------------------------------

await init();

for (const id of ["steam-t", "steam-p"]) $(id).addEventListener("input", renderSteam);
for (const id of ["sec-shape", "sec-w", "sec-h", "sec-tf", "sec-tw"]) {
  $(id).addEventListener("input", renderSection);
}
for (const id of ["pid-kp", "pid-ki", "pid-kd", "pid-tau", "pid-sp", "pid-lim"]) {
  $(id).addEventListener("input", renderPid);
}

renderSteam();
renderSection();
renderPid();
