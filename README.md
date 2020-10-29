# Splinter

**`Splinter` is a spline library with proven and tested curve and spline algorithms.**

---

## Motiviation

There are a lot of resources out there that provide information on quadratic and cubic curves, but many of them contain information that either:

- is simply incorrect
- is correct but not proven
- is correct and proven but inefficient

The goal of `splinter` is to provide not only a set of robust and efficient algorithms, but to also provide proofs along the way.

## Highlights

`Splinter` provides functions and proofs for:

- optimal approximations of higher-order curves by lower-order curves, with error margins
- a closed-form solution for the length of a quadratic curve
- an exact calculation of the nearest point on a quadratic curve
- a robust iterative algorithm to parameterize a quadratic curve by length

## Quadratics vs Cubics

When possible, `splinter` will provide optimal algorithms for both quadratic and cubic curves. However, when such algorithms are not available it is typically best to choose error margins, subdivide, and approximate the cubic with quadratics.

## Where are the proofs?

Proofs are located in markdown files, split up by category. Look in `cubic_curves.md` and `quadratic_curves.md`. You'll want a markdown viewer that supports rendering LaTeX math.

## License

Licensed under the [MIT license](https://github.com/djkoloski/splinter/blob/master/LICENSE).