# Day 1: The Tyranny of the Rocket Equation

## Part One

### Problem

<https://adventofcode.com/2019/day/1>

Santa has become stranded at the edge of the Solar System while delivering presents to other planets! To accurately calculate his position in space, safely align his warp drive, and return to Earth in time to save Christmas, he needs you to bring him measurements from `fifty stars`.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants `one star`. Good luck!

The Elves quickly load you into a spacecraft and prepare to launch.

At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper. They haven't determined the amount of fuel required yet.

Fuel required to launch a given **module** is based on its **mass**. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.

For example:

- For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
- For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
- For a mass of 1969, the fuel required is 654.
- For a mass of 100756, the fuel required is 33583.
- The Fuel Counter-Upper needs to know the total fuel requirement. To find it, individually calculate the fuel needed for the mass of each module (your puzzle input), then add together all the fuel values.

**What is the sum of the fuel requirements** for all of the modules on your spacecraft?

### Input

<https://adventofcode.com/2019/day/1/input>

136583
77036
109200
142168
74357
146941
129306
98180
105195
129127
135956
116070
89198
51306
144552
109900
56658
52478
115147
63882
70342
98678
107384
135359
87237
84469
106477
104645
77066
74143
76537
140547
68128
116624
148407
78276
117623
96019
75825
75010
98644
119641
139736
104452
72599
63017
59648
126163
69629
79080
92195
58221
134276
80301
89870
146799
145702
138367
131977
56781
85326
138115
70241
60454
76934
119321
93493
123047
149941
141729
70141
134525
93312
92043
79582
115959
51058
94686
70749
99408
118560
95821
58995
94906
98421
118673
83575
83434
63884
70575
134177
116233
113954
52829
123860
128020
126718
144463
140192
143461

### Answer

> 3366415

is the total fuel required.

## Part Two

### Problem

During the second Go / No Go poll, the Elf in charge of the Rocket Equation Double-Checker stops the launch sequence. Apparently, you forgot to include additional fuel for the fuel you just added.

Fuel itself requires fuel just like a module - take its mass, divide by three, round down, and subtract 2. However, that fuel **also** requires fuel, and **that** fuel requires fuel, and so on. Any mass that would require **negative fuel** should instead be treated as if it requires **zero fuel**; the remaining mass, if any, is instead handled by **wishing really hard**, which has no mass and is outside the scope of this calculation.

So, for each module mass, calculate its fuel and add it to the total. Then, treat the fuel amount you just calculated as the input mass and repeat the process, continuing until a fuel requirement is zero or negative. For example:

- A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
- At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
- The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.

**What is the sum of the fuel requirements** for all of the modules on your spacecraft when also taking into account the mass of the added fuel? (Calculate the fuel requirements for each module separately, then add them all up at the end.)

> Note: This problem uses the same input as Part One above

### Input

This problem uses the same input as Part One above.

### Answer
