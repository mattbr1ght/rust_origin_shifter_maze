# hello,

this is a maze solver, generator and loader.

# Purpose

This is a hobby project that later evolved to support loading, and flags for one of my CS classes.

# Philosophy

I strive to write modular and general code and this project is designed with that in mind. I want people to be able to easily understand the logic, and swap out modules of my code that they want kinda like modding.

# Windows CMD not displaying colors?

to fix it you need to create the following DWORD key in registry

```
[HKEY_CURRENT_USER\Console]
"VirtualTerminalLevel"=dword:00000001
```

# Gallery

```
origin_shifter.exe -W15 -H10 -i
```
![image](https://github.com/user-attachments/assets/0ec4d6bc-bc40-4921-ae83-35e744cb93ef)

after moving a bit and toggling the visibility of path and origin

![image](https://github.com/user-attachments/assets/3b38bf98-246d-4fd7-b1b7-9155c64f90df)

---
**WITH SHIFTING**
---

```
origin_shifter.exe -W15 -H10 -id -sO -n 1000
```
![image](https://github.com/user-attachments/assets/92427c5f-b075-44ce-9a82-c0a87aa6c54b)

the next image shows how the maze was scrumbled in 1000 shifts per 1 move of the player

![image](https://github.com/user-attachments/assets/9f16c08a-cc67-4aa0-a80a-0d2b0ddcdd69)


let's try with smaller shifts per move number

```
origin_shifter.exe -W15 -H10 -id -sO -n5
```

![image](https://github.com/user-attachments/assets/4edf4a3f-03cb-4d59-982e-595d23e5f7ba)

after one move the path didn't change (direction of the shift is random so it can move in ways that already have passages and essentially not mutate the maze)

![image](https://github.com/user-attachments/assets/537d7bb4-1170-4320-95a6-6d945de1f8ef)

but after another 2 moves the path was changed so that we need to go back in order to reach the end

![image](https://github.com/user-attachments/assets/0132a44d-3e36-455a-98e9-81250feb3add)

---
**BACKSTAGE**
---

this is how the program recognizes nodes. they are just lists of 4 booleans stored in a vector that symbolize directions in which they can connect with each other (it can lead to redundancy and it does in my approach but it's fine for this project). eg. NEIGHBOURS[0] is a list of 4 booleans that looks like so [false, false, true, true] (NOTE: i start with the left direction and go clockwise); this is true for the following image.

![image](https://github.com/user-attachments/assets/f653a700-05ba-4adc-9d6d-bc3756e05be6)

---
**FILE READING**
---

when you run the following command the program will look for maze.txt file and load it. it must have fixed width and height (so a rectangle) have sides made of walls and passages marked as `.` like maze.txt in the repo. 

#NOTE: as of now player ALWAYS starts in the upper left corner and the finish node is in the bottom right but i plan on adding the functionality to modify player's starting position and finish node position

```
origin_shifter.exe -f maze.txt
```

![image](https://github.com/user-attachments/assets/2c1b1a82-19b1-4aad-9b03-2478d83d63ca)

the above is a representation recreated from the following text file:

```
#################
#.#.......#.....#
#.#.###.#####.###
#.#.#.....#.#...#
#.###.#.###.#.#.#
#.#.#.#.#.#...#.#
#.#.###.#.#.###.#
#.....#.#.....#.#
#.#####.#.#.#.###
#...#.....#.#...#
#.#####.#########
#.#.#...........#
#.#.#########.###
#.....#.....#...#
#.#.#.#####.#.###
#.#.#.#.....#...#
#.###.###.#####.#
#...#...........#
#################
```

---
**SHADOWS/FOG**
---

you can add the `-S` flag to enable shadowing/fog in the game. it looks like the following

```
origin_shifter.exe -f maze.txt -S
```

![image](https://github.com/user-attachments/assets/9a57e102-3806-491f-9e05-505e6b025c5c)

![image](https://github.com/user-attachments/assets/63f79c65-3b56-4a1f-89bc-0d26d8c4b54b)

you can controll the view distance by specifying the `-D` option 

```
origin_shifter.exe -f maze.txt -S -D6
```

![image](https://github.com/user-attachments/assets/3b923cc7-1d49-4d17-8ba9-0770f8f69fb4)

![image](https://github.com/user-attachments/assets/675adf55-6fee-416c-a4f3-011becbd6c5c)

as you see high values can result in unrealistic views so i suggest keeping it as high as 4 or less

