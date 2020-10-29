# Quadratic Curves

## Definition

Given a vector space $V$:

$$ f(t) = x_0 + x_1 t + x_2 t^2 $$
$$ x_0, x_1, x_2 \in V $$

## Derivatives

$$ f'(t) = x_1 + 2 x_2 t $$
$$ f''(t) = 2 x_2 $$

## From Bezier control points

Quadratic Bezier curves are defined by the formula:

$$ B(t) = (1 - t)^2 b_0 + 2 (1 - t) t b_1 + t^2 b_2 $$
$$ b_0, b_1, b_2 \in V $$

Where $b_0$ is the starting point, $b_2$ is the ending point, and $b_1$ is the control point. Expand to obtain:

$$ B(t) = b_0 - 2 b_0 t + b_0 t^2 + 2 b_1 t - 2 b_1 t^2 + b_2 t^2 $$
$$ B(t) = b_0 + (-2 b_0 + 2 b_1) t + (b_0 - 2 b_1 + b_2) t^2 $$

Giving:

$$ x_0 = b_0 $$
$$ x_1 = 2 (b_1 - b_0) $$
$$ x_2 = b_0 - 2 b_1 + b_2 $$

In matrix form:

$$
\begin{bmatrix}
    x_0 \\
    x_1 \\
    x_2
\end{bmatrix}
=
\begin{bmatrix}
    1 & 0 & 0 \\
    -2 & 2 & 0 \\
    1 & -2 & 1
\end{bmatrix}
\begin{bmatrix}
    b_0 \\
    b_1 \\
    b_2
\end{bmatrix}
$$

## To Bezier control points

Inverting the above equations:

$$ b_0 = x_0 $$
$$ b_1 = x_0 + \frac{x_1}{2} $$
$$ b_2 = x_0 + x_1 + x_2 $$

In matrix form:

$$
\begin{bmatrix}
    1 & 0 & 0 \\
    -2 & 2 & 0 \\
    1 & -2 & 1
\end{bmatrix}
^{-1}
\begin{bmatrix}
    x_0 \\
    x_1 \\
    x_2
\end{bmatrix}
=
\begin{bmatrix}
    b_0 \\
    b_1 \\
    b_2
\end{bmatrix}
$$
$$
\begin{bmatrix}
    1 & 0 & 0 \\
    -2 & 2 & 0 \\
    1 & -2 & 1
\end{bmatrix}
^{-1}
=
\begin{bmatrix}
    1 & 0 & 0 \\
    1 & \frac{1}{2} & 0 \\
    1 & 1 & 1
\end{bmatrix}
$$

## Interval reversal

Given a quadratic curve $f(t)$:

$$ f(t) = x_0 + x_1 t + x_2 t^2 $$

The quadratic curve identical to $f(t)$ with the endpoints $t=0,1$ reversed is:

$$ g(t) = f(1 - t) $$
$$ g(t) = y_0 + y_1 t + y_2 t^2 $$

Where:

$$ y_0 = x_0 + x_1 + x_2 $$
$$ y_1 = - x_1 - 2 x_2 $$
$$ y_2 = x_2 $$

## Subdivision

Let $u$ be some value at which to subdivide the curve $f(t), 0 \leq t \leq 1$ into two curves $f_0(t)$ and $f_1(t)$:

$$ f(t) = x_0 + x_1 t + x_2 t^2 $$

$f_0(t)$ is the first segment ranging from $0$ to $u$:

$$ f_0(t) = f(ut) = x_0 + u x_1 t + u^2 x_2 t^2 $$

$f_0(t)$ takes the form:

$$ f_0(t) = y_0 + y_1 t + y_2 t^2 $$

Where:

$$ y_0 = x_0 $$
$$ y_1 = u x_1 $$
$$ y_2 = u^2 x_2 $$

$f_1(t)$ is the second segment ranging from $u$ to $1$:

$$ f_1(t) = f(u + (1 - u) t) = x_2 (u^2 + 2 u (1 - u) t + (1 - u)^2 t^2) + x_1 (u + (1 - u)t) + x_0 $$
$$ f_1(t) = u^2 x_2 + u x_1 + x_0 + (2 u (1 - u) x_2 + (1 - u) x_1) t + (1 - u)^2 x_2 t^2 $$

$f_1(t)$ takes the form:

$$ f_1(t) = y_0 + y_1 t + y_2 t^2 $$

Where:

$$ y_0 = x_0 + u x_1 + u^2 x_2 $$
$$ y_1 = (1 - u) x_1 + 2 u (1 - u) x_2 $$
$$ y_2 = (1 - u)^2 x_2 $$

## Calculating the length

Let $l(u)$ be the length of a quadratic $f(t)$ between $0$ and $u$:

$$ l(u) = \int_0^u \sqrt{f'(t) \cdot f'(t)} dt $$

Expand the first derivative of the quadratic:

$$ f'(t) = x_1 + 2 x_2 t $$

Giving:

$$ l(u) = \int_0^u \sqrt{(x_1 + 2 x_2 t) \cdot (x_1 + 2 x_2 t)} dt $$
$$ l(u) = \int_0^u \sqrt{x_1 \cdot x_1 + 4 (x_1 \cdot x_2) t + 4 (x_2 \cdot x_2) t^2} dt $$

Which is of the form:

$$ l(u) = \int_0^u \sqrt{a t^2 + b t + c} dt $$

Where:

$$ a = 4 (x_2 \cdot x_2) $$
$$ b = 4 (x_1 \cdot x_2) $$
$$ c = x_1 \cdot x_1 $$

### $a = 0$

If $a = 0$ then $b = 0$ as well, and we are left with:

$$ l(u) = \int_0^u \sqrt c dt = u \sqrt c $$

### $a \neq 0$

Depress the quadratic with the transformation:

$$ x = t \sqrt a + \frac{b}{2 \sqrt a} $$
$$ dx = \sqrt a dt $$
$$ k = c - \frac{b^2}{4a} $$

Into:

$$ l(u) = \frac{1}{\sqrt a} \int_{\frac{b}{2 \sqrt a}}^{u \sqrt a + \frac{b}{2 \sqrt a}} \sqrt{x^2 + k} dx $$

Which allows us to use the identity:

$$ \int \sqrt{x^2 \pm k} dx = \frac{1}{2} \left( x \sqrt{x^2 \pm k} \pm k \ln \left| x + \sqrt{x^2 \pm k} \right| \right) $$

Giving:

$$ l(u) = \frac{1}{2 \sqrt a} \left. \left( x \sqrt{x^2 + k} + k \ln \left| x + \sqrt{x^2 + k} \right| \right) \right| _{\frac{b}{2 \sqrt a}}^{u \sqrt a + \frac{b}{2 \sqrt a}} $$

### Full algorithm

$$ z = \sqrt a $$
$$ n = \frac{1}{2 z} $$
$$ p_0 = b n $$
$$ p_1 = u z + p_0 $$
$$ k = c - p_0^2 $$
$$ s_0 = \sqrt{c} $$
$$ s_1 = \sqrt{p_1^2 + k} $$
$$
l(u) =
\begin{cases}
    a = 0 & u s_0 \\
    k = 0 & n \left( p_1 s_1 - p_0 s_0 \right) \\
    k \neq 0 & n \left( p_1 s_1 - p_0 s_0 + k \ln \left| \frac{p_1 + s_1}{p_0 + s_0} \right| \right)
\end{cases}
$$

## Length parameterization

Using the length function $l(u)$ above, given some $v$, find $u$ such that:

$$ l(u) = v $$

Because $l(u)$ is not algebraically invertible, a root-finding approach must be used instead:

$$ l(u) - v = 0 $$

Understanding the properties of $l(u)$ is necessary to determine an appropriate approximation method.

* $l(u)$ monotonicly increases
* $l(u) - v$ has no zeroes when $v < 0$ and one zero when $v \geq 0$

A hybrid between Newton's method and root bracketing will yield fast convergence for most solutions while ensuring that the solution does not diverge for the others.

```
parameterize(v, tolerance, max_iters):
    l = 0
    fl = -v
    r = 1
    fr = length(1) - v
    x = l - (r - l) * fl / (fr - fl)

    if fl > 0:
        return 0
    else if fr < 0:
        return 1
    else:
        loop max_iters times:
            fx = length(x) - v
            if |fx| < tolerance:
                break

            if fx > 0:
                r = x
                fr = fx
            else:
                l = x
                fl = fx

            x -= fx / speed(x)
            if x < l or x > r:
                x = l - (r - l) * fl / (fr - fl)
```

For lengths that lie outside of the bounds of the curve ($v < 0$ or $v > l(1)$), this returns a clamped result of $0$ or $1$.

There exists a degenerative case where the curve does not move and always has a length of $0$ when:

$$ x_1 \cdot x_1 = x_2 \cdot x_2 = 0 $$

In this case, it's the discretion of the implementor what to return.

## Calculating the nearest point on the curve

Let $q$ be some point in $V$:

$$ q \in V $$

Let $f(t)$ be a quadratic curve:

$$ f(t) = x_0 + x_1 t + x_2 t^2 $$

Translate the quadratic curve by $-q$ to obtain a new curve $g(t)$:

$$ g(t) = x_0 - q + x_1 t + x_2 t^2 $$

Let $d(t)$ be the distance from the origin to the point $g(t)$:

$$ d(t) = \sqrt{g(t) \cdot g(t)} $$

$d(t)$ takes the form:

$$ d(t) = \sqrt{a t^4 + b t^3 + c t^2 + d t + e} $$

Where:

$$ a = x_2 \cdot x_2 $$
$$ b = 2(x_1 \cdot x_2) $$
$$ c = 2((x_0 - q) \cdot x_2) + x_1 \cdot x_1 $$
$$ d = 2 ((x_0 - q) \cdot x_1) $$
$$ e = (x_0 - q) \cdot (x_0 - q) $$

Derive to obtain $d'(t)$:

$$ d'(t) = \frac{4 a t^3 + 3 b t^2 + 2 c t + d}{2 \sqrt{a t^4 + b t^3 + c t^2 + d t + e}} $$

Which has zeroes where:

$$ 4 a t^3 + 3 b t^2 + 2 c t + d = 0 $$

Solve the cubic, obtaining up to three roots:

$$ d'(r_0) = d'(r_1) = d'(r_2) = 0 $$

Calculate $d(t)$ for each root and endpoint of the curve (e.g. $0$ and $1$), and return the value of $u$ which yielded the lowest value of $d(t)$.

## Approximation by a linear curve

Given a quadratic curve $f(t)$:

$$ f(t) = x_0 + x_1 t + x_2 t^2 $$

There are three main approximations using a linear curve: error-minimizing, continuity-preserving, and left-point. Each imposes different constraints on the solution and so gives a different error term.

### Error-minimizing

The unique linear curve that minimizes the error takes the form:

$$ g(t) = y_0 + y_1 t $$

Where:

$$ y_0 = x_0 - \frac{1}{8} x_2 $$
$$ y_1 = x_1 + x_2 $$

With a maximum error of:

$$ max( | f(t) - g(t) | ) = \frac{1}{8} | x_2 | = 0.125 | x_2 | $$

at:

$$ t = 0, \frac{1}{2}, 1 $$

and an error of $0$ at:

$$ t = \frac{2 \pm \sqrt{2}}{4} $$

and a total error of:

$$ \int_0^1 | f(t) - g(t) | dt = \frac{2 \sqrt{2} - 1}{24} | x_2 | \approx 0.0761844 | x_2 | $$

### Continuity-preserving (endpoint approximation)

The unique linear curve that satisfies:

$$ g(0) = f(0) $$
$$ g(1) = f(1) $$

And minimizes the error takes the form:

$$ g(t) = y_0 + y_1 t $$

Where:

$$ y_0 = x_0 $$
$$ y_1 = x_1 + x_2 $$

With a maximum error of:

$$ max( | f(t) - g(t) | ) = \frac{1}{4} | x_2 | = 0.25 | x_2 | $$

at:

$$ t = \frac{1}{2} $$

and an error of $0$ at:

$$ t = 0, 1 $$

and a total error of:

$$ \int_0^1 | f(t) - g(t) | dt = \frac{1}{6} | x^2 | \approx 0.1666667 | x_2 | $$

### Left point approximation (half continuity-preserving)

The unique linear curve that satisfies:

$$ g(0) = f(0) $$

And minimizes the error takes the form:

$$ g(t) = y_0 + y_1 t $$

Where:

$$ y_0 = x_0 $$
$$ y_1 = x_1 + (2 \sqrt{2} - 2) x_2 $$

With a maximum error of:

$$ max( | f(t) - g(t) | ) = 3 - 2 \sqrt{2} \approx 0.1715728 | x_2 | $$

at:

$$ t = \sqrt{2} - 1, 1 $$

and a total error of:

$$ \int_0^1 | f(t) - g(t) | dt = \frac{37 \sqrt{2} - 52}{3} | x_2 | \approx  0.1086339 | x_2 | $$