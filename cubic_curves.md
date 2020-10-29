# Cubic curves

## Definition

Given a vector space $V$:

$$ f(t) = x_0 + x_1 t + x_2 t^2 + x_3 t^3 $$
$$ x_0, x_1, x_2, x_3 \in V $$

## Derivatives

$$ f'(t) = x_1 + 2 x_2 t + 3 x_3 t^2 $$
$$ f''(t) = 2 x_2 + 6 x_3 t $$
$$ f'''(t) = 6 x_3 $$

## From Bezier control points

Cubic bezier curves are defined by the formula:

$$ B(t) = (1 - t)^3 b_0 + 3 (1 - t)^2 t b_1 + 3 (1 - t) t^2 b_2 + t^3 b_3 $$

Where $b_0$ is the starting point, $b_3$ is the ending point, $b_1$ is the control point for $b_0$, and $b_2$ is the control point for $b_3$. Expand to obtain:

$$ B(t) = b_0 - 3 t b_0 + 3 t^2 b_0 - t^3 b_0 + 3 t b_1 - 6 t^2 b_1 + 3 t^3 b_1 + 3 t^2 b_2 - 3 t^3 b_2 + t^3 b_3 $$
$$ B(t) = b_0 + (-3 b_0 + 3 b_1) t + (3 b_0 - 6 b_1 + 3 b_2) t^2 + (-b_0 + 3 b_1 - 3 b_2 + b_3) t^3 $$

Giving:

$$ x_0 = b_0 $$
$$ x_1 = 3 (b_1 - b_0) $$
$$ x_2 = 3 (b_0 - 2 b_1 + b_2) $$
$$ x_3 = 3 (b_1 - b_2) - b_0 + b_3 $$

In matrix form:

$$
\begin{bmatrix}
    x_0 \\
    x_1 \\
    x_2 \\
    x_3
\end{bmatrix}
=
\begin{bmatrix}
    1 & 0 & 0 & 0 \\
    -3 & 3 & 0 & 0 \\
    3 & -6 & 3 & 0 \\
    -1 & 3 & -3 & 1
\end{bmatrix}
\begin{bmatrix}
    b_0 \\
    b_1 \\
    b_2 \\
    b_3
\end{bmatrix}
$$

## To Bezier control points:

Inverting the above equations:

$$ b_0 = x_0 $$
$$ b_1 = x_0 + \frac{x_1}{3} $$
$$ b_2 = x_0 + \frac{2 x_1 + x_2}{3} $$
$$ b_3 = x_0 + x_1 + x_2 + x_3 $$

In matrix form:

$$
\begin{bmatrix}
    1 & 0 & 0 & 0 \\
    -3 & 3 & 0 & 0 \\
    3 & -6 & 3 & 0 \\
    -1 & 3 & -3 & 1
\end{bmatrix}
^{-1}
\begin{bmatrix}
    x_0 \\
    x_1 \\
    x_2 \\
    x_3
\end{bmatrix}
=
\begin{bmatrix}
    b_0 \\
    b_1 \\
    b_2 \\
    b_3
\end{bmatrix}
$$
$$
\begin{bmatrix}
    1 & 0 & 0 & 0 \\
    -3 & 3 & 0 & 0 \\
    3 & -6 & 3 & 0 \\
    -1 & 3 & -3 & 1
\end{bmatrix}
^{-1}
=
\begin{bmatrix}
    1 & 0 & 0 & 0 \\
    1 & \frac{1}{3} & 0 & 0 \\
    1 & \frac{2}{3} & \frac{1}{3} & 0 \\
    1 & 1 & 1 & 1
\end{bmatrix}
$$

## Subdivision

Let $u$ be some value at which to subdivide the curve $f(t), 0 \leq t \leq 1$ into two curves $f_0(t)$ and $f_1(t)$:

$$ f(t) = x_0 + x_1 t + x_2 t^2 + x_3 t^4 $$

$f_0(t)$ is the first segment ranging from $0$ to $u$:

$$ f_0(t) = f(ut) = x_0 + u x_1 t + u^2 x_2 t^2 + u^3 x_3 t^3 $$

$f_0$ takes the form:

$$ f_0(t) = y_0 + y_1 t + y_2 t^2 + y_3 t^3 $$

Where:

$$ y_0 = x_0 $$
$$ y_1 = u x_1 $$
$$ y_2 = u^2 x_2 $$
$$ y_3 = u^3 x_3 $$

$f_1(t)$ is the second segment ranging from $u$ to $1$:

$$ f_1(t) = f(u + (1 - u)t) = x_3 (u^3 + 3 u^2 (1 - u) t + 3 u (1 - u)^2 t^2 + (1 - u)^3 t^3) + x_2 (u^2 + 2 u (1 - u) t + (1 - u)^2 t^2) + x_1 (u + (1 - u) t) + x_0 $$
$$ f_1(t) = u^3 x_3 + u^2 x_2 + u x_1 + x_0 + (3 u^2 (1 - u) + 2 u (1 - u) x_2 + (1 - u) x_1) t + (3 u (1 - u)^2 x_3 + (1 - u)^2 x_2) t^2 + (1 - u)^3 x_3 t^3 $$

$f_1(t)$ takes the form:

$$ f_1(t) = y_0 + y_1 t + y_2 t^2 + y_3 t^3 $$

Where:

$$ y_0 = u^3 x_3 + u^2 x_2 + u x_1 + x_0 $$
$$ y_1 = 3 u^2 (1 - u) x_3 + 2 u (1 - u) x_2 + (1 - u) x_1 $$
$$ y_2 = 3 u (1 - u)^2 x_3 + (1 - u)^2 x_2 $$
$$ y_3 = (1 - u)^3 x_3 $$

## Approximation by a quadratic curve

Given a cubic curve $f(t)$:

$$ f(t) = x_0 + x_1 t + x_2 t^2 + x_3 t^3 $$

There are two main quadratic curve approximations:

### Error-minimizing

The unique quadratic curve that minimizes the error takes the form:

$$ g(t) = y_0 + y_1 t + y_2 t^2 $$

Where:

$$ y_0 = x_0 + \frac{1}{32} x_3 $$
$$ y_1 = x_1 - \frac{9}{16} x_3 $$
$$ y_2 = x_2 + \frac{3}{2} x_3 $$

With a maximum error of:

$$ max( | f(t) - g(t) | ) = \frac{1}{32} | x_3 | $$

at:

$$ t = 0, \frac{1}{4}, \frac{3}{4}, 1 $$

and an error of $0$ at:

$$ t = \frac{1}{2}, \frac{1}{4}(2 \pm \sqrt{3}) $$

and a total error of:

$$ \int_0^1 | f(t) - g(t) | dt = \frac{5}{256} | x_3 | $$

### Continuity-preserving (midpoint approximation)

The unique quadratic curve that satisfies:

$$ g(0) = f(0) $$
$$ g(1) = f(1) $$

And minimizes the error takes the form:

$$ g(t) = y_0 + y_1 t + y_2 t^2 $$

Where:

$$ y_0 = x_0 $$
$$ y_1 = x_1 - \frac{1}{2} x_3 $$
$$ y_2 = x_2 + \frac{3}{2} x_3 $$

With a maximum error of:

$$ max( | f(t) - g(t) | ) = \frac{1}{12 \sqrt{3}} | x_3 | $$

at:

$$ t = \frac{1}{2} \pm \frac{1}{2 \sqrt{3}} $$

and an error of $0$ at:

$$ t = 0, \frac{1}{2}, 1 $$

and a total error of:

$$ \int_0^1 | f(t) - g(t) | dt = \frac{1}{32} | x_3 | $$

## Approximation by multiple quadratic curves

Given the cubic:

$$ f(t) = x_0 + x_1 t + x_2 t^2 + x_3 t^3 $$

Subdivide the function multiple times and approximate each piece with a quadratic. The best approximation technique to use depends on the application:

* When calculating the length of a cubic, it is best to approximate using the error-minimizing quadratic
* When converting a cubic to multiple quadratics for display, it is best to approximate using the continuity-preserving quadratic

For either, the cubic must be subdivided at some location $t_0$. The optimal choice of $t_0$ for both approximation techniques is:

$$ t_0 = \frac{1}{2} $$

$t_0$ must be chosen to minimize:

$$ err(f(t_0 t)) + err(f(t_0 + (1 - t_0) t)) $$

Both approximations have an error term that is proportional to the coefficient of $t^3$, so minimizing the sum of the resulting coefficients will yield the ideal choice of $t_0$. By choosing $\frac{1}{2}$, the resulting error terms are:

$$ err \left( f \left( \frac{1}{2} t \right) \right) = k \left( \frac{1}{2} \right) ^3 x_3 $$
$$ err \left( f \left( \frac{1}{2} + \frac{1}{2} t \right) \right) = k \left( \frac{1}{2} \right) ^3 x_3 $$

Where $k$ is the error factor of the approximation:

$$ k = \frac{1}{32}, \frac{1}{12 \sqrt{3}} $$

Increasing or decreasing $t_0$ would increase the sum of the error terms. By repeatedly subdividing the curve at $t_0$, the error term of the quadratic approximation will decrease by a factor of $\frac{1}{8}$ each time.