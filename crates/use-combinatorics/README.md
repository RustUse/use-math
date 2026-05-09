# use-combinatorics

<p align="center">
    <strong>Checked combinatorics helpers for explicit counting in `u128`.</strong><br>
    Factorials, permutations, and combinations with structured errors instead of wrapping arithmetic.
</p>

<p align="center">
    <img alt="Rust 1.95.0+" src="https://img.shields.io/badge/Rust-1.95.0%2B-f46623?logo=rust&logoColor=white">
    <img alt="Edition 2024" src="https://img.shields.io/badge/edition-2024-0f766e">
    <img alt="Arithmetic u128" src="https://img.shields.io/badge/arithmetic-u128-1d4ed8">
    <img alt="Checked errors" src="https://img.shields.io/badge/errors-explicit-c2410c">
    <img alt="License MIT or Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2a9d8f">
</p>

<p align="center">
    <a href="#what-this-crate-provides">Surface</a> ·
    <a href="#when-to-use-it-directly">When to use it</a> ·
    <a href="#installation">Installation</a> ·
    <a href="#quick-examples">Examples</a> ·
    <a href="#error-and-overflow-model">Errors</a> ·
    <a href="#scope">Scope</a>
</p>

`use-combinatorics` provides overflow-aware counting helpers as focused utility building blocks for common tasks such as factorials, ordered selections, and unordered selections. The crate stays intentionally small: explicit inputs, exact results when they fit in `u128`, and explicit error values when they do not.

<table>
    <tr>
        <td width="33%" valign="top">
            <strong>Factorials</strong><br>
            <code>factorial(n)</code> returns <code>n!</code> with checked <code>u128</code> arithmetic.
        </td>
        <td width="33%" valign="top">
            <strong>Permutations</strong><br>
            <code>permutations(n, k)</code> counts ordered selections and rejects <code>k &gt; n</code>.
        </td>
        <td width="33%" valign="top">
            <strong>Combinations</strong><br>
            <code>combinations(n, k)</code> counts unordered selections and cancels factors before multiply where possible.
        </td>
    </tr>
</table>

## What this crate provides

| Helper               | Meaning                                          | Failure mode                                                          |
| -------------------- | ------------------------------------------------ | --------------------------------------------------------------------- |
| `factorial(n)`       | Counts total orderings of `n` items              | `FactorialOverflow(n)` when the exact result no longer fits in `u128` |
| `permutations(n, k)` | Counts ordered selections of size `k` from `n`   | `KExceedsN { n, k }` or `PermutationOverflow { n, k }`                |
| `combinations(n, k)` | Counts unordered selections of size `k` from `n` | `KExceedsN { n, k }` or `CombinationOverflow { n, k }`                |

| If you need to...                                        | Start here                             |
| -------------------------------------------------------- | -------------------------------------- |
| Count without bringing in geometry APIs                  | `use-combinatorics`                    |
| Keep overflow explicit instead of saturating or wrapping | Any public helper in this crate        |
| Count actual arrangements or subsets, not just totals    | Another crate or your own domain logic |
| Work past `u128` limits with big integers                | Another crate or a future extension    |

## When to use it directly

Choose `use-combinatorics` directly when checked counting is the only math support you need, or when you want the narrowest possible dependency and API surface.

| Scenario                                                | Use `use-combinatorics` directly? | Why                                                         |
| ------------------------------------------------------- | --------------------------------- | ----------------------------------------------------------- |
| You only need factorials, permutations, or combinations | Yes                               | The crate is tiny and purpose-built                         |
| You want explicit overflow errors in application logic  | Yes                               | All helpers return `Result<u128, CombinatoricsError>`       |
| You also need geometry types and helpers                | Usually no                        | `use-math` may be the cleaner integration surface           |
| You need arbitrary-precision combinatorics              | No                                | This crate intentionally stops at checked `u128` arithmetic |

## Installation

```toml
[dependencies]
use-combinatorics = "0.0.1"
```

## Quick examples

### Basic checked counting

```rust
use use_combinatorics::{combinations, factorial, permutations};

assert_eq!(factorial(5)?, 120);
assert_eq!(permutations(5, 3)?, 60);
assert_eq!(combinations(5, 2)?, 10);
assert_eq!(combinations(52, 5)?, 2_598_960);
# Ok::<(), use_combinatorics::CombinatoricsError>(())
```

### Invalid input stays explicit

```rust
use use_combinatorics::{CombinatoricsError, combinations, permutations};

assert_eq!(
        combinations(3, 4),
        Err(CombinatoricsError::KExceedsN { n: 3, k: 4 })
);
assert_eq!(
        permutations(3, 4),
        Err(CombinatoricsError::KExceedsN { n: 3, k: 4 })
);
```

### Overflow is reported, not hidden

```rust
use use_combinatorics::{CombinatoricsError, factorial, combinations};

assert_eq!(factorial(35), Err(CombinatoricsError::FactorialOverflow(35)));
assert_eq!(
        combinations(150, 75),
        Err(CombinatoricsError::CombinationOverflow { n: 150, k: 75 })
);
```

## Error and overflow model

All public helpers return `Result<u128, CombinatoricsError>` so invalid inputs and arithmetic overflow remain visible to callers.

| Concern                | Behavior                                                                |
| ---------------------- | ----------------------------------------------------------------------- |
| Invalid selection size | `k > n` returns `KExceedsN { n, k }`                                    |
| Factorial growth       | `factorial` fits through `n = 34`, then returns `FactorialOverflow`     |
| Permutation growth     | Overflow depends on both `n` and `k`, returned as `PermutationOverflow` |
| Combination growth     | Overflow depends on both `n` and `k`, returned as `CombinationOverflow` |

> [!NOTE]
> `combinations` cancels common factors before multiplying so more exact results fit in `u128` without changing the mathematical answer.

## Scope

- Small APIs are preferred over broad trait-heavy abstractions.
- Counting helpers stay explicit about invalid inputs and arithmetic overflow.
- Big-integer fallbacks are intentionally out of scope for now.
- Enumerating actual arrangements or subsets is intentionally out of scope.
- Probability distributions and random sampling helpers are intentionally deferred.
- Additional combinatorics utilities can grow incrementally from this base.

## Status

`use-combinatorics` is a concrete pre-1.0 crate in the `RustUse` docs surface. The API remains intentionally small, and the `RustUse`-hosted generated rustdocs stay canonical while external crates.io and docs.rs pages remain staged.
