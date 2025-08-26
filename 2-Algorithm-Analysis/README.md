# Algorithm Analysis

## The RAM Model of Computation
Machine-independent algorithm design depends upon a hypothetical computer called *Random Access Machine*, or RAM. Under this model of computation, we are confronted with a computer where:
* Each *simple* operation (+, -, /, \*, =, `if`, call) takes exactly one time step.
* Loops and subroutines are not considered simple operations. They are the composition of many single-step operations.
* Each memory access takes one time step.

But on real computers those three rules may not apply. On many processors multiplying takes more time then adding two numbers; the process of loops a can break the second point on fancy compilers; the time taken fo memory access is different depending where the data is in the storaged hierarchy.

Every model in science has a size range on which is usefull. The flat Earth model for example, it is well known that the Earth is flat, but when building a house the flat Earth model is of very good use, since it is not necessary to take the curvature of the Earth in count. For the RAM model is the same, is an abstraction that is generally very usefull.

(The Earth is not completely spherical either, but a spherical Earth provides a useful model for such things as longitude and latitude.)

## Best-Case, Worst-Case and Average-Case Complexity
The Worst-Case scenario proves to be the most useful of these three, which seems counterintuive. To ilustrate why, think on the situation where you go a casino with a *n* amount of money. THe best-case scenario is when you go out of the place owning it, very unlikely; the worst-case scenario is when loose everithing, most likely to happen and easier to calculate.

## The Big Oh Notation
Intead of working with precise equations and formulas for time-complexity function analysis, is much easier to talk in terms of simple upper and lower bounds function with the Big OH notation.

## Growth Rates and Dominance Relations
List of classes of functions in dominance order (g >> f):
* **Constant functions:** f(n) = 1
* **Logarithmic functions:** f(n) = log n
* **Linear functions:** f(n) = n
* **Superlinear functions:** f(n) = n * log n
* **Quadratic functions:** f(n) = n^2
* **Cubic functions:** f(n) = n^3
* **Exponential functions:** f(n) = c^n
* **Factorial functions:** f(n) = n!

> n! >> c^n >> n^3 >> n^2 >> n * log n >> n >> log n >> 1

## Logarithm
Two implications of logarithm's properties are important to appreciate from an algorithmic perspective:
* *The base of logarithm has no real impact on the growth rate*
* *Logarithms cut any function down to size*: The growth rate of any polinomial function is O(lg n)
