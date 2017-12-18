When compiled with total memory under 1 wasm page, the JavaScript console reads:


```
Gamestate pointer: 1179256
Called draw with counter:  1
Called draw with ptr: 1114248
data[ 0 ]: 43
data[ 1 ]: 43
data[ 2 ]: 5
data[ 3 ]: 5
data[ 4 ]: 5
Called draw with counter:  2
Called draw with ptr: 1114248
data[ 0 ]: 44
data[ 1 ]: 44
data[ 2 ]: 5
data[ 3 ]: 5
data[ 4 ]: 5
Called draw with counter:  3
Called draw with ptr: 1114248
data[ 0 ]: 45
data[ 1 ]: 45
data[ 2 ]: 5
data[ 3 ]: 5
data[ 4 ]: 5
```

(as expected)

When compiled with say `67_000`, I get this instead:


```
Gamestate pointer: 1181256
Called draw with counter:  1
Called draw with ptr: 1114248
data[ 0 ]: 0
data[ 1 ]: 0
data[ 2 ]: 0
data[ 3 ]: 0
data[ 4 ]: 0
Called draw with counter:  2
Called draw with ptr: 1114248
data[ 0 ]: 0
data[ 1 ]: 0
data[ 2 ]: 0
data[ 3 ]: 0
data[ 4 ]: 0
Called draw with counter:  3
Called draw with ptr: 1114248
data[ 0 ]: 0
data[ 1 ]: 0
data[ 2 ]: 0
data[ 3 ]: 0
data[ 4 ]: 0
```

In other words, neither the initial Vec value (`5`) nor the new one
(`42` plus `counter`) is displayed.

Note however that the `counter` value is still being updated though.
