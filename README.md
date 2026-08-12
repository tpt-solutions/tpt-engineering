# tpt-eng-standards

Standards modeling as **structured, parameterized data** for the TPT engineering
ecosystem.

> **Legal rule.** This crate contains **no** copyrighted standard text, no
> proprietary code clauses, and no scraped proprietary tables. It models the
> *shape* of standards-based calculations — load cases, load combinations,
> load/safety factors, and limit-state logic — as generic, user-filled data
> structures. The actual factors and combinations are supplied by the user (who
> is responsible for the license of the values they enter).

## Modules

- [`load`](load) — [`LoadCase`](load::LoadCase) and [`LoadType`](load::LoadType).
- [`combinations`](combinations) — [`LoadCombination`](combinations::LoadCombination)
  and the arithmetic to evaluate one against a demand map. Combinations and their
  factors are user-provided data.
- [`factors`](factors) — [`FactorSet`](factors::FactorSet), a user-supplied bag
  of named partial factors.
- [`limit_states`](limit_states) — [`LimitState`](limit_states::LimitState) and
  the parameterized [`DemandCapacity`](limit_states::DemandCapacity)
  utilization check.
- [`design`](design) — the [`DesignBasis`](design::DesignBasis) aggregate and the
  [`evaluate_check`](design::evaluate_check)/[`CheckResult`](design::CheckResult)
  workflow that ties a combination to a limit state and a capacity.

## User-provided data

Every factor, combination, and factor set is plain data the caller provides. See
the module examples for building a [`DesignBasis`](design::DesignBasis) from your
own (correctly licensed) values.

## License

Dual-licensed under [MIT](../../LICENSE-MIT) OR
[Apache-2.0](../../LICENSE-APACHE).
