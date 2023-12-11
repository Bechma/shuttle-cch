Shuttle's Christmas Code Hunt
=============================

### [🎄 Day 1: Packet "exclusive-cube" not found](src/days/day_01.rs)

In the frosty expanses of the North Pole, Santa's advanced packet management system has encountered a glitch. This
system, known for its robustness and magical speed, is responsible for sorting and dispatching all the Christmas
gifts. However, a sudden aurora borealis storm has scattered bits of data across the snowy landscape, leaving Santa in
dire need of a digital handyman to restore order.

⭐ Task 1: Cube the bits

Santa needs your programming expertise to recalibrate the packet IDs in his packet management system.

Implement a GET endpoint /1/<num1>/<num2> that takes 2 integers in the path, num1 and num2, and returns the result of (
num1 XOR num2) POW 3, where XOR is the exclusive OR operation, and POW is exponentiation.

💠 Example

```bash
curl http://localhost:8000/1/4/8

1728
```

🎁 Task 2: The sled ID system (100 bonus points)

After the packet IDs are calibrated and the packets are packed into sleds, Santa needs to calibrate the sled ID.

The formula is very similar: All packet IDs (integers) are XOR'ed with each other, and then the result is (again) raised
to the power of 3. The catch is that there can be between 1 and 20 packets in a sled!

Adapt the endpoint from Task 1 so that it can also be used to calculate sled IDs.

💠 Examples

```bash
curl http://localhost:8000/1/10

1000
```

```bash
curl http://localhost:8000/1/4/5/8/10

27
```

### Day 2 and Day 3 missing due to some issues in shuttle

### [🎄 Day 4: What do you call a serialized reindeer? Serdeer!](src/days/day_04.rs)

Under the soft glow of the Northern Lights, Santa's reindeer are training for the big night. But, oh deer! The
reindeer's stats have been serialized into an unknown format after a playful elf accidentally spilled hot cocoa on the
central computer. The data needs to be deserialized so the reindeer team can be assembled based on their combined
strength.

⭐ Task 1: Reindeer cheer

The task is to create a POST endpoint /4/strength that calculates the combined strength of a group of reindeer, so that
Santa knows if they can pull his sled through the skies.

The input to the endpoint is a JSON array containing information about each reindeer. Each reindeer is represented as an
object with two attributes: "name" (string) and "strength" (integer). Collect the strength of each reindeer and respond
with the sum.

💠 Example

```bash
curl -X POST http://localhost:8000/4/strength \
-H 'Content-Type: application/json' \
-d '[
    { "name": "Dasher", "strength": 5 },
    { "name": "Dancer", "strength": 6 },
    { "name": "Prancer", "strength": 4 },
    { "name": "Vixen", "strength": 7 }
]'

22
```

🎁 Task 2: Cursed candy eating contest (150 bonus points)

This time, there is some more data for each reindeer (see example). The endpoint is called /4/contest, because the
reindeer are now going to compare the attributes of the reindeer and present a summary of the winners.

There is at least one reindeer participating in the contest, and there is never a tie for first place.

For some reason, one of the field names in the input seems to still be a bit corrupted from the incident. Guess we just
have to deal with it anyways.

The output should be a JSON object containing a summary of the winners (see example).

💠 Example Input

```bash
curl -X POST http://localhost:8000/4/contest \
-H 'Content-Type: application/json' \
-d '[
    {
        "name": "Dasher",
        "strength": 5,
        "speed": 50.4,
        "height": 80,
        "antler_width": 36,
        "snow_magic_power": 9001,
        "favorite_food": "hay",
        "cAnD13s_3ATeN-yesT3rdAy": 2
    },
    {
        "name": "Dancer",
        "strength": 6,
        "speed": 48.2,
        "height": 65,
        "antler_width": 37,
        "snow_magic_power": 4004,
        "favorite_food": "grass",
        "cAnD13s_3ATeN-yesT3rdAy": 5
    }
]'
```

💠 Example Output

Note: JSON output examples are sometimes formatted. Output from your endpoint does not need to be formatted. The output
is parsed and checked as a value.

```json
{
  "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
  "tallest": "Dasher is standing tall with his 36 cm wide antlers",
  "magician": "Dasher could blast you away with a snow magic power of 9001",
  "consumer": "Dancer ate lots of candies, but also some grass"
}
```

### 🎄 Day 5?

The Grinch stole the contents of this challenge while the Shuttle team wasn't looking! Check back soon in case we find
the challenge contents!

### [🎄 Day 6: Elf on a shelf](src/days/day_06.rs)

It's that time of year when the elves hide on shelves to watch over the children of the world, reporting back to Santa
on who's been naughty or nice. However, this year's reports have been mixed up with the rest of the letters to Santa,
and the word "elf" is hidden throughout a mountain of text.

⭐ Task 1: Never count on an elf

Elves are notorious for their hide-and-seek skills, and now they've hidden themselves in strings of text!

Create an endpoint /6 that takes a POST request with a raw string as input and count how many times the substring "elf"
appears.

The output should be a JSON object containing the count of the string "elf".

💠 Examples

```bash
curl -X POST http://localhost:8000/6 \
-H 'Content-Type: text/plain' \
-d 'The mischievous elf peeked out from behind the toy workshop,
and another elf joined in the festive dance.
Look, there is also an elf on that shelf!'

{"elf":4}
```

🎁 Task 2: Shelf under an elf? (200 bonus points)

Add two fields to the response that counts:

The number of occurrences of the string "elf on a shelf" in a field with the same name.
The number of shelves that don't have an elf on it. That is, the number of strings "shelf" that are not preceded by the
string "elf on a ". Put this count in the field "shelf with no elf on it".

💠 Example

```bash
curl -X POST http://localhost:8000/6 \
-H 'Content-Type: text/plain' \
-d 'there is an elf on a shelf on an elf.
there is also another shelf in Belfast.'

{"elf":5,"elf on a shelf":1,"shelf with no elf on it":1}
```

### [🎄 Day 7: GET Santa some cookies](src/days/day_07.rs)

At Santa's base near the North Pole (64 km away to be precise), the scent of freshly baked cookies fills the air, a sign
that Christmas is near. Santa, however, has forgotten the encoding method that was used to hide his favorite cookie
recipe in web browsers around the world. He needs this super-secret recipe (based on his grandfather's recipe made in
1964!) to fuel his late-night toy-making sessions.

⭐ Task 1: Based encoding, 64th edition

Santa's secret cookie recipe is hidden in an encoded message, and he's looking to you for help. He's sending a GET
request to your server with a Cookie header.

What you need to do is parse the Cookie header, decode the value in the recipe field, and return it.

Make an endpoint /7/decode that extracts the Cookie HTTP header. The header in the request will look something like
this:

`Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==`

After decoding the recipe value to bytes, convert it to a string and return it as the response to the GET request.

💠 Example

```bash
curl http://localhost:8000/7/decode \
-H 'Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ=='

{"flour":100,"chocolate chips":20}
```

🎁 Task 2: The secret cookie recipe (120 bonus points)

Now that the recipe is decoded, Santa can get back to baking cookies! Santa is not sure, however, if he has enough of
each ingredient to bake a cookie for every elf.

The same type of request as in Task 1 will be sent to a new endpoint, /7/bake, but this time Santa needs your help to
calculate the amount of cookies he can bake with the ingredients he has in the pantry.

After decoding, parse the bytes as a JSON object (shape and keys can be seen in the example below) and use that to
calculate how many cookies can be baked with the provided recipe and ingredients. Additionally, return the amount of
ingredients that would remain in the pantry after the cookies have been baked.

💠 Example Input

```bash
curl http://localhost:8000/7/bake \
-H 'Cookie: recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319'
```

After decoding, the recipe above will look like this JSON object:

```json
{
  "recipe": {
    "flour": 95,
    "sugar": 50,
    "butter": 30,
    "baking powder": 10,
    "chocolate chips": 50
  },
  "pantry": {
    "flour": 385,
    "sugar": 507,
    "butter": 2122,
    "baking powder": 865,
    "chocolate chips": 457
  }
}
```

💠 Example Output

```json
{
  "cookies": 4,
  "pantry": {
    "flour": 5,
    "sugar": 307,
    "butter": 2002,
    "baking powder": 825,
    "chocolate chips": 257
  }
}
```

Explanation: The recipe represents the required ingredients to make one cookie. After baking 4 cookies, we have 5 units
of flour left and can't bake any more.

🎁 Task 3: Questionable cookie recipes (100 bonus points)

Some mischievous elves have now found your endpoint, and are trying their "innovative" cookie recipes on it, without
even knowing what ingredients are available in the pantry!

Update the endpoint from Task 2 so that any set of ingredients can be present in the recipe and the pantry,
respectively.

The number of cookies in the answer will always be finite.

💠 Example

```bash
curl http://localhost:8000/7/bake \
-H 'Cookie: recipe=eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ=='

{
    "cookies": 0,
    "pantry": {
      "cobblestone": 64,
      "stick": 4
    }
}
```

### [🎄 Day 8: PokéPhysics](src/days/day_08.rs)

In the heart of the North Pole, Santa's workshop buzzes with a new type of magic. A portal has opened, and Pokémon from
another world have tumbled into the snow-dusted realm of elves and reindeer. Santa, ever the innovator, sees an
opportunity: why not enlist these charming creatures in his annual gift-giving campaign?

But before the sleigh bells ring and the Pokémon can join the flight, Santa needs to understand their unique properties
and figure out how many can fit into his sleigh, given their weight.

⭐ Task 1: IT'S PIKACHU!

Your quest is to add a GET endpoint /8/weight/<pokedex_number> that, given a pokédex number, responds with the
corresponding Pokémon's weight in kilograms as a number in plain text.

💠 Example

```bash
curl http://localhost:8000/8/weight/25

6
```

🎁 Task 2: That's gonna leave a dent (160 bonus points)

Once the Pokémon's weight is retrieved, Santa needs you to calculate the momentum it would have at the time of impact
with the floor if dropped from a `10-meter` high chimney (so that he knows if he needs to climb down or if he can just
drop it).

Keep in mind that the gravity of Earth that Santa has in his physics book was measured close to the North Pole. This
could explain why his calculations are a bit off sometimes, but he still wants you to use it.

The momentum, measured in Newton-seconds, signifies the force the Pokémon would exert upon meeting the floor beneath the
10-meter high chimney.

The GET endpoint /8/drop/<pokedex_number> shall respond with a plain text floating point number.

Use gravitational acceleration `g = 9.825 m/s²`. Ignore air resistance.

💠 Example

```bash
curl http://localhost:8000/8/drop/25

84.10707461325713
```

Validation has a fault tolerance of 0.001

### [🎄 Day 11: Imagery from the North Pole](src/days/day_11.rs)

Decked out in his signature red coat, Santa's eyes sparkle brighter than the Northern Star as he navigates through tall
shelves packed with newly produced Christmas decorations for the season. Handcrafted glass balls, ornate stars,
whimsical snowflakes, festive tinsel - you name it, they have it all.

⭐ Task 1: Served on a silver platter

The time has come to decorate our surroundings! The elves are getting tired of working with just strings and numbers and
bytes, and are in need of some fancy christmas ornaments on the computer screens.

![image provided](assets/decoration.png)

Download the image above and serve it as a static file so that a GET request to /11/assets/decoration.png responds with
the image file and a correct MIME type header (Content-Type: image/png).

💠 Example Input

```bash
curl -I -X GET http://localhost:8000/11/assets/decoration.png
```

💠 Example Output

```
HTTP/1.1 200 OK
content-type: image/png
content-length: 787297
...
```

🎁 Task 2: Bull mode activated (200 bonus points)

Add a POST endpoint /11/red_pixels, that takes in a PNG image in the image field of a multipart POST request, and
returns the number of pixels in the image that are perceived as "magical red" when viewed with Santa's night vision
goggles. The goggles considers a pixel "magical red" if the color values of the pixel fulfill the formula red > blue +
green.

💠 Example

```bash
curl -X POST http://localhost:8000/11/red_pixels \
-H 'Content-Type: multipart/form-data' \
-F 'image=@decoration.png' # the image from Task 1

73034
```
