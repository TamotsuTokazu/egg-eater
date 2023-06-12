# Green-egg-eater Write-up
Hello instructors! Thank you for the fantastic course!

This is my write-up for Green-egg-eater, which implements some extension for egg-eater. This write-up is organized as follows:

- The first part is the write-up for the extensions, in addition to the original write-up.
- The second part is the original write-up for egg-eater refined, with more explanations and some issues fixed.

### Some random ideas about structural equality

After implementing structural equality, I really feel that this concept makes more sense when the objects are immutable. In this case, we can totally justify `a: (x x)` and `b: (x y)` are structurally equal (assume `x` and `y` are structurally equal). However, if `x` is mutable, we can actually distinguish `a` from `b` by something like `a[0].foo()` and `b[0].foo()`, where in `a` both fields are changed, and in `b` only the first field is changed.