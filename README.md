Shuttle's Christmas Code Hunt
=============================

# Index:

- [🎄 Day 1: Packet "exclusive-cube" not found](#-day-1-packet-exclusive-cube-not-found)
- [Day 2 and Day 3 missing due to some issues in shuttle](#day-2-and-day-3-missing-due-to-some-issues-in-shuttle)
- [🎄 Day 4: What do you call a serialized reindeer? Serdeer!](#-day-4-what-do-you-call-a-serialized-reindeer-serdeer)
- [🎄 Day 5: Why did Santa's URL query go haywire on Christmas? Too many "present" parameters!](#-day-5-why-did-santas-url-query-go-haywire-on-christmas-too-many-present-parameters-)
- [🎄 Day 6: Elf on a shelf](#-day-6-elf-on-a-shelf)
- [🎄 Day 7: GET Santa some cookies](#-day-7-get-santa-some-cookies)
- [🎄 Day 8: PokéPhysics](#-day-8-poképhysics)
- [🎄 Day 11: Imagery from the North Pole](#-day-11-imagery-from-the-north-pole)
- [🎄 Day 12: Timekeeper](#-day-12-timekeeper)
- [🎄 Day 13: Santa's Gift Orders](#-day-13-santas-gift-orders)
- [🎄 Day 14: Reindeering HTML](#-day-14-reindeering-html)
- [🎄 Day 15: The Password Validator](#-day-15-the-password-validator)
- [🎄 Day 18: Santa's Gift Orders: Data Analytics Edition](#-day-18-santas-gift-orders-data-analytics-edition)
- [🎄 Day 19: Christmas Sockets on the Chimney](#-day-19-christmas-sockets-on-the-chimney)
- [🎄 Day 20: Git good](#-day-20-git-good)
- [🎄 Day 21: Around the Globe](#-day-21-around-the-globe)
- [🎄 Day 22: Dawn of the day before the day before the final day](#-day-22-dawn-of-the-day-before-the-day-before-the-final-day)

## [🎄 Day 1: Packet "exclusive-cube" not found](src/days/day_01.rs)

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

## Day 2 and Day 3 missing due to some issues in shuttle

## [🎄 Day 4: What do you call a serialized reindeer? Serdeer!](src/days/day_04.rs)

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

## [🎄 Day 5: Why did Santa's URL query go haywire on Christmas? Too many "present" parameters!](src/days/day_05.rs)

In the technologically advanced North Pole, Santa decided to streamline his gift-tracking system using URL query
parameters, entrusting the elves with entering present requests. However, the mischievous Grinch added duplicate
parameters like "present=puzzle" and "present=unicorn" as a prank. On Christmas Eve, as Santa set out to deliver gifts,
the excess parameters caused a glitch: the list of names entered an infinite loop.

⭐ Task 1: Slicing the Loop

Santa has some lists of names that are becoming too long to deal with. Help him by adding URL query parameters for
paginating the list.

The task is to create a POST endpoint /5 that takes a JSON list of names, and query parameters offset and limit as
numbers. Then, return the sub-slice of the list between index offset and offset + limit.

💠 Example

```bash
curl -X POST "http://localhost:8000/5?offset=3&limit=5" \
-H 'Content-Type: application/json' \
-d '[
"Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe",
"Nolan", "Harper", "Lucas", "Stella", "Mason", "Olivia"
]'

["Owen", "Lily", "Ethan", "Zoe", "Nolan"]
```

🎁 Task 2: Time to Page Some Names (150 bonus points)

This time, Santa also needs to be able to get all pages at once.

Modify the same endpoint, so that it can also handle a split parameter. All parameters should now be optional. If not
given, offset defaults to 0, and limit defaults to including all remaining items in the list. If split is not given, no
splitting will happen, but if given, the output list should be split into sub-lists with length according the the value.

💠 Example

```bash
curl -X POST http://localhost:8000/5?split=4 \
-H 'Content-Type: application/json' \
-d '[
"Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe",
"Nolan", "Harper", "Lucas", "Stella", "Mason", "Olivia"
]'

[
["Ava", "Caleb", "Mia", "Owen"],
["Lily", "Ethan", "Zoe", "Nolan"],
["Harper", "Lucas", "Stella", "Mason"],
["Olivia"]
]
```

```bash
curl -X POST "http://localhost:8000/5?offset=5&split=2" \
-H 'Content-Type: application/json' \
-d '[
"Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe",
"Nolan", "Harper", "Lucas", "Stella", "Mason", "Olivia"
]'

[
["Ethan", "Zoe"],
["Nolan", "Harper"],
["Lucas", "Stella"],
["Mason", "Olivia"]
]
```

## [🎄 Day 6: Elf on a shelf](src/days/day_06.rs)

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

## [🎄 Day 7: GET Santa some cookies](src/days/day_07.rs)

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

## [🎄 Day 8: PokéPhysics](src/days/day_08.rs)

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

## [🎄 Day 11: Imagery from the North Pole](src/days/day_11.rs)

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

## [🎄 Day 12: Timekeeper](src/days/day_12.rs)

One frosty night, Santa, dressed warmly in his favorite red coat, decided to take a midnight stroll around the elf
workshop. As he pushed open the heavy wooden doors of the workshop, his eyes widened in surprise. He was completely
stunned by the sight that greeted him.

Rows upon rows of conveyor belts had been set up, zipping toys from one corner to the other, resembling an intricate
dance of festivity and efficiency. The elves were operating with military precision, organizing toys into specific
categories and sending them down the right pathways.

⭐ Task 1: How To Time Persist? (HTTP)

Presents are being packed and wrapped at blazingly fast speeds in the workshop. In order to gather data on the
production of presents, Santa needs a multi-stopwatch that can keep the time of many packet IDs at once.

Create two endpoints:

POST /12/save/<string>: takes a string and stores it.
GET /12/load/<string>: takes the same string and returns the number of whole seconds elapsed since the last time it was
stored.

💠 Example

```bash
curl -X POST http://localhost:8000/12/save/packet20231212
sleep 2
curl http://localhost:8000/12/load/packet20231212
echo
sleep 2
curl http://localhost:8000/12/load/packet20231212
echo
curl -X POST http://localhost:8000/12/save/packet20231212
curl http://localhost:8000/12/load/packet20231212

# After ~4 seconds:
2
4
0
```

🎁 Task 2: Unanimously Legendary IDentifier (ULID) (100 bonus points)

Santa, who likes old-school tech, now sees that some packets use modern ULIDs. Help him rewind time a little bit by
showing him them in an older format that he understands.

Make a POST endpoint /12/ulids that takes a JSON array of ULIDs. Convert all the ULIDs to UUIDs and return a new array
but in reverse order.

💠 Example

```bash
curl -X POST http://localhost:8000/12/ulids \
-H 'Content-Type: application/json' \
-d '[
        "01BJQ0E1C3Z56ABCD0E11HYX4M",
        "01BJQ0E1C3Z56ABCD0E11HYX5N",
        "01BJQ0E1C3Z56ABCD0E11HYX6Q",
        "01BJQ0E1C3Z56ABCD0E11HYX7R",
        "01BJQ0E1C3Z56ABCD0E11HYX8P"
    ]'

[
    "015cae07-0583-f94c-a5b1-a070431f7516",
    "015cae07-0583-f94c-a5b1-a070431f74f8",
    "015cae07-0583-f94c-a5b1-a070431f74d7",
    "015cae07-0583-f94c-a5b1-a070431f74b5",
    "015cae07-0583-f94c-a5b1-a070431f7494"
]
```

🎁 Task 3: Let Santa Broil (LSB) (200 bonus points)

Now that Santa is up to date on some newer data formats, he needs help with analyzing the manufacturing date of some
packets he found in the corner of the workshop.

Create another variant of the same endpoint /12/ulids/<weekday> that counts the number of ULIDs that fulfill the
following criteria (in the UTC timezone):

How many of the ULIDs were generated on a Christmas Eve?
How many were generated on a <weekday>? (A number in the path between 0 (Monday) and 6 (Sunday))
How many were generated in the future? (has a date later than the current time)
How many have entropy bits where the Least Significant Bit (LSB) is 1?

💠 Example

```bash
curl -X POST http://localhost:8000/12/ulids/5 \
-H 'Content-Type: application/json' \
-d '[
        "00WEGGF0G0J5HEYXS3D7RWZGV8",
        "76EP4G39R8JD1N8AQNYDVJBRCF",
        "018CJ7KMG0051CDCS3B7BFJ3AK",
        "00Y986KPG0AMGB78RD45E9109K",
        "010451HTG0NYWMPWCEXG6AJ8F2",
        "01HH9SJEG0KY16H81S3N1BMXM4",
        "01HH9SJEG0P9M22Z9VGHH9C8CX",
        "017F8YY0G0NQA16HHC2QT5JD6X",
        "03QCPC7P003V1NND3B3QJW72QJ"
    ]'

{
    "christmas eve": 3,
    "weekday": 1,
    "in the future": 2,
    "LSB is 1": 5
}
```

## [🎄 Day 13: Santa's Gift Orders](src/days/day_13.rs)

Santa Claus has started facing a pressing issue at the North Pole. The existing database, written in a legacy language,
is becoming insufficient for handling the tidal wave of gift requests from children worldwide. This ancient system is
not only slowing down operations, but it is also proving harder to maintain.

To ensure that not a single child's wish is overlooked and operations run as efficiently as possible, an immediate
upgrade is a necessity.

⭐ Task 1: SQL? Sequel? Squeel??

Santa's gift order database is written in an ancient language and needs to be oxidized. Let's show him the power of Rust
with your backend combined with a Postgres database.

Add a Postgres database with the Shuttle Shared Database plugin, and add the pool to your application state. Add a GET
endpoint /13/sql that executes the SQL query SELECT 20231213 and responds with the query result (an i32 turned into a
string).

💠 Example

```bash
curl http://localhost:8000/13/sql

20231213
```

⭐ Task 2: Use code NorthPole2023 for 2023% off???

Now that the data can be migrated over to the new database, we see that Santa's workshop has received numerous gift
orders from different regions. Time to do some basic analysis.

Create a POST endpoint /13/reset that (re-)creates the following schema in your database upon being called, and returns
a plain 200 OK. It will be used at the start of each test to ensure a clean starting point.

```sql
DROP TABLE IF EXISTS orders;
CREATE TABLE orders
(
    id        INT PRIMARY KEY,
    region_id INT,
    gift_name VARCHAR(50),
    quantity  INT
);
```

Then, create a POST endpoint /13/orders that takes a JSON array of order objects and inserts them into the table (see
below). Return a plain 200 OK.

Lastly, create a GET endpoint /13/orders/total that queries the table and returns the total number of gifts ordered (the
sum of all quantities).

💠 Example

```bash
curl -X POST http://localhost:8000/13/reset
curl -X POST http://localhost:8000/13/orders \
-H 'Content-Type: application/json' \
-d '[
{"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
{"id":2,"region_id":2,"gift_name":"Doll","quantity":8},
{"id":3,"region_id":3,"gift_name":"Action Figure","quantity":12},
{"id":4,"region_id":4,"gift_name":"Board Game","quantity":10},
{"id":5,"region_id":2,"gift_name":"Teddy Bear","quantity":6},
{"id":6,"region_id":3,"gift_name":"Toy Train","quantity":3}
]'
curl http://localhost:8000/13/orders/total

{"total":44}
```

🎁 Task 3: Truly one of the gifts of all time (100 bonus points)

Add a GET endpoint /13/orders/popular that returns the name of the most popular gift. If there is no most popular gift,
use null instead of a string.

```bash
curl -X POST http://localhost:8000/13/reset
curl -X POST http://localhost:8000/13/orders \
-H 'Content-Type: application/json' \
-d '[
{"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
{"id":2,"region_id":2,"gift_name":"Doll","quantity":8},
{"id":3,"region_id":3,"gift_name":"Toy Train","quantity":4}
]'
curl http://localhost:8000/13/orders/popular

{"popular":"Toy Train"}
```

## [🎄 Day 14: Reindeering HTML](src/days/day_14.rs)

Did you hear about the time when Santa became a web designer? He picked up coding with great enthusiasm. Each tag told a
story, every element was a toy, and every attribute was a wish from a child around the world. He soon managed to build a
website where children could easily send their letters filled with Christmas wishes, and the elves could more
efficiently organize the toymaking process.

⭐ Task 1: Ho-ho, Toymaking Magic Land! (HTML)

Today we are simulating an incident that happened shortly after Santa joined the web dev team at the North Pole.

Implement a POST endpoint /14/unsafe that takes some HTML content and unsafely renders it on a small HTML page.

🔔 Tips

If you choose to use a templating engine for this task, make sure you disable escaping to allow unsafe rendering.

💠 Example Input

```bash
curl -X POST http://localhost:8000/14/unsafe \
-H "Content-Type: application/json" \
-d '{"content": "<h1>Welcome to the North Pole!</h1>"}'
```

💠 Example Output

Make sure that no extra whitespace is rendered. The response content below is 124 bytes long.

```html

<html>
<head>
    <title>CCH23 Day 14</title>
</head>
<body>
<h1>Welcome to the North Pole!</h1>
</body>
</html>
```

🎁 Task 2: Safety 2nd (100 bonus points)

Time to clean up the mess that Santa caused in Task 1. Show him how it's done in /14/safe by securely rendering the HTML
against script injection.

💠 Example Input

```bash
curl -X POST http://localhost:8000/14/safe \
-H "Content-Type: application/json" \
-d '{"content": "<script>alert(\"XSS Attack!\")</script>"}'
```

💠 Example Output

```html

<html>
<head>
    <title>CCH23 Day 14</title>
</head>
<body>
&lt;script&gt;alert(&quot;XSS Attack!&quot;)&lt;/script&gt;
</body>
</html>
```

## [🎄 Day 15: The Password Validator](src/days/day_15.rs)

There had been a few naughty incidents where mischievous users had tinkered with other children's wish lists, not
respecting the goodwill and spirit of the platform. It was clear: the website needed to add a layer of security to
protect the integrity of each child's wish list.

The team behind the scenes, a dedicated crew of Santa's tech-savvy elves (led by you), rolled up their sleeves. They
decided to implement a homemade password validation system that ensured that no Grinch could easily guess or crack the
password.

⭐ Task 1: Naughty or Nice Strings

Now that children can sign up to the letter sending website, we need an endpoint for validating their passwords.

Create an endpoint at `/15/nice` that accepts POST requests with a JSON payload containing a password string to
validate.

The rules at this endpoint are:

- Nice Strings: Must contain at least three vowels (`aeiouy`), at least one letter that appears twice in a row, and must
  not contain the substrings: `ab`, `cd`, `pq`, or `xy`.
- Naughty Strings: Do not meet the criteria for nice strings.
  Return an appropriate HTTP status code and message (see below) indicating whether the provided string is nice or
  naughty.

💠 Examples

```bash
curl -X POST http://localhost:8000/15/nice \
-H 'Content-Type: application/json' \
-d '{"input": "hello there"}'

# 200 OK
{"result":"nice"}
```

```bash
curl -X POST http://localhost:8000/15/nice \
-H 'Content-Type: application/json' \
-d '{"input": "abcd"}'

# 400 Bad Request
{"result":"naughty"}
```

```bash
curl -X POST http://localhost:8000/15/nice \
-H 'Content-Type: application/json' \
-d '{Grinch? GRINCH!}'

# 400 Bad Request
# response body does not matter
```

🎁 Task 2: Game of the Year (400 bonus points)

Santa thought this validation thing was so fun that it could be turned into a game!

Add a similar endpoint, POST /15/game, that has this set of rules:

Nice Strings: Must adhere to all the rules:

- Rule 1: must be at least 8 characters long
- Rule 2: must contain uppercase letters, lowercase letters, and digits
- Rule 3: must contain at least 5 digits
- Rule 4: all integers (sequences of consecutive digits) in the string must add up to 2023
- Rule 5: must contain the letters `j`, `o`, and `y` in that order and in no other order
- Rule 6: must contain a letter that repeats with exactly one other letter between them (like `xyx`)
- Rule 7: must contain at least one unicode character in the range `[U+2980, U+2BFF]`
- Rule 8: must contain at least one emoji
- Rule 9: the hexadecimal representation of the sha256 hash of the string must end with an `a`

Naughty Strings: Do not meet the criteria for nice strings.
Check the string for the rules in the listed order. Return the corresponding status code and reason (and naughty/nice
result) based on which rule was violated:

| Rule broken | Status Code | Reason                 |
|-------------|-------------|------------------------|
| 1           | 400         | 8 chars                |
| 2           | 400         | more types of chars    |
| 3           | 400         | 55555                  |
| 4           | 400         | math is hard           |
| 5           | 406         | not joyful enough      |
| 6           | 451         | illegal: no sandwich   |
| 7           | 416         | outranged              |
| 8           | 426         | 😳                     |
| 9           | 418         | not a coffee brewer    |
| None        | 200         | that's a nice password |

💠 Examples

```bash
curl -X POST http://localhost:8000/15/game \
-H 'Content-Type: application/json' \
-d '{"input": "password"}'

# 400 Bad Request

{"result":"naughty","reason":"more types of chars"}
```

```bash
curl -X POST http://localhost:8000/15/game \
-H 'Content-Type: application/json' \
-d '{"input": "Password12345"}'

# 400 Bad Request
{"result":"naughty","reason":"math is hard"}
```

```bash
curl -X POST http://localhost:8000/15/game \
-H 'Content-Type: application/json' \
-d '{"input": "23jPassword2000y"}'

# 451 Unavailable For Legal Reasons
{"result":"naughty","reason":"illegal: no sandwich"}
```

## [🎄 Day 18: Santa's Gift Orders: Data Analytics Edition](src/days/day_18.rs)

Santa sat back in his plush seat, a mug of hot cocoa in his hand, and a smile on his jolly face. The database upgrade
from the previous week had indeed worked out exceptionally well; the operations were running smoother than ever, the
reports were accurate, and morale among his helpers was at an all-time high. This modern marvel of technology had
infused a new spirit into the North Pole operations.

⭐ Task 1: Mr. Worldwide

This challenge continues from what was built for the Core tasks on Day 13.

Santa is stoked about the speed and reliability of the new gift order database backend! He wants you to expand it to
support per-region analytics.

Copy the /13/reset endpoint from Day 13 to /18/reset, but modify the query like this:

```sql
DROP TABLE IF EXISTS regions;
DROP TABLE IF EXISTS orders;

CREATE TABLE regions
(
    id   INT PRIMARY KEY,
    name VARCHAR(50)
);

CREATE TABLE orders
(
    id        INT PRIMARY KEY,
    region_id INT,
    gift_name VARCHAR(50),
    quantity  INT
);
```

We want to re-use the POST endpoint /13/orders at /18/orders for adding new orders. You can either add the same handler
under the new route, or just copy+paste the entire thing, as long as both endpoints are doing the same thing.

Now, add a POST endpoint /18/regions that inserts regions in the same way the orders endpoint does.

Lastly, add a GET endpoint /18/regions/total that returns the total number of orders per region. To make it easier for
Santa to find a location, the output should be alphabetically sorted on the region name. Regions with no orders should
not be listed in the result.

💠 Example

```bash
curl -X POST http://localhost:8000/18/reset
curl -X POST http://localhost:8000/18/regions \
-H 'Content-Type: application/json' \
-d '[
  {"id":1,"name":"North Pole"},
  {"id":2,"name":"Europe"},
  {"id":3,"name":"North America"},
  {"id":4,"name":"South America"},
  {"id":5,"name":"Africa"},
  {"id":6,"name":"Asia"},
  {"id":7,"name":"Oceania"}
]'
curl -X POST http://localhost:8000/18/orders \
-H 'Content-Type: application/json' \
-d '[
  {"id":1,"region_id":2,"gift_name":"Board Game","quantity":5},
  {"id":2,"region_id":2,"gift_name":"Origami Set","quantity":8},
  {"id":3,"region_id":3,"gift_name":"Action Figure","quantity":12},
  {"id":4,"region_id":4,"gift_name":"Teddy Bear","quantity":10},
  {"id":5,"region_id":2,"gift_name":"Yarn Ball","quantity":6},
  {"id":6,"region_id":3,"gift_name":"Art Set","quantity":3},
  {"id":7,"region_id":5,"gift_name":"Robot Lego Kit","quantity":5},
  {"id":8,"region_id":6,"gift_name":"Drone","quantity":9}
]'
curl http://localhost:8000/18/regions/total

# [
#   {"region":"Africa","total":5},
#   {"region":"Asia","total":9},
#   {"region":"Europe","total":19},
#   {"region":"North America","total":15},
#   {"region":"South America","total":10}
# ]
```

🎁 Task 2: West Pole to East Pole - Santa wants ALL the data (600 bonus points)

To optimize production of gifts for next year, Santa needs detailed insights into the best performing gifts in every
region.

Create a GET endpoint /18/regions/top_list/<number> that retrieves the names of the regions along with the top <number>
most ordered gifts in each region, considering the quantity of orders placed for each gift.

If there are less than <number> unique gifts in a region, the top list will be shorter. If there are no gifts in a
region, show that with an empty top list.

If there is a tie among gifts, use alphabetical ordering of the gift name to break it. The final output shall once again
be ordered by region name.

💠 Example

```bash
curl -X POST http://localhost:8000/18/reset

curl -X POST http://localhost:8000/18/regions \
-H 'Content-Type: application/json' \
-d '[
  {"id":1,"name":"North Pole"},
  {"id":2,"name":"South Pole"},
  {"id":3,"name":"Kiribati"},
  {"id":4,"name":"Baker Island"}
]'

curl -X POST http://localhost:8000/18/orders \
-H 'Content-Type: application/json' \
-d '[
  {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
  {"id":2,"region_id":2,"gift_name":"Toy Train","quantity":3},
  {"id":3,"region_id":2,"gift_name":"Doll","quantity":8},
  {"id":4,"region_id":3,"gift_name":"Toy Train","quantity":3},
  {"id":5,"region_id":2,"gift_name":"Teddy Bear","quantity":6},
  {"id":6,"region_id":3,"gift_name":"Action Figure","quantity":12},
  {"id":7,"region_id":4,"gift_name":"Board Game","quantity":10},
  {"id":8,"region_id":3,"gift_name":"Teddy Bear","quantity":1},
  {"id":9,"region_id":3,"gift_name":"Teddy Bear","quantity":2}
]'
curl http://localhost:8000/18/regions/top_list/2

# [
#   {"region":"Baker Island","top_gifts":["Board Game"]},
#   {"region":"Kiribati","top_gifts":["Action Figure","Teddy Bear"]},
#   {"region":"North Pole","top_gifts":[]},
#   {"region":"South Pole","top_gifts":["Doll","Toy Train"]}
# ]
```

## [🎄 Day 19: Christmas Sockets on the Chimney](src/days/day_19.rs)

On a cold and snowy winter day, Santa Claus was busy with his annual routine when he spotted a new delivery of vibrant
socks hanging on his chimney. The hues and prints on these socks were unlike anything he had seen before - intricate
patterns with tiny paddles embroidered on them. He chuckled, remembering how he used to juggle between writing protocols
for his websocket apps and practising his backhand strokes on his virtual table tennis game.

⭐ Task 1: Table Tennis Server 🏓

Write a WebSocket GET endpoint /19/ws/ping that listens for messages of type Text.

If the incoming string is serve, the game starts in this WebSocket.
If and only if the game has started, respond with a string pong whenever the incoming string is ping.
All other incoming messages should be ignored.

💠 Example

curl is not sufficient for testing WebSocket behavior with simple commands. Use the official validator (link at bottom
of page) to run local tests for this challenge.

🎁 Task 2: Bird App Simulator (500 bonus points)

To improve internal communications at the North Pole, Santa is trying out a real-time variant of Twitter (sometimes
referred to as a "chat app"). (Santa is old-school & cool - still calls it Twitter instead of X).

In order to know how much the elves are using the platform, Santa wants some metrics. He thinks it is sufficient to just
count the total number of views on all tweets.

Here are the required endpoints:

- POST endpoint /19/reset that resets the counter of tweet views.
- GET endpoint /19/views that returns the current count of tweet views.
- GET endpoint /19/ws/room/<number>/user/<string> that opens a WebSocket and connects a user to a room.

This is how the app should work:

- A user can at any time send a tweet as a Text WebSocket message in the format `{"message":"Hello North Pole!"}`.
- When a tweet is received, broadcast it to everyone in the same room (including the sender).
- Tweets with more than 128 characters are too long and should be ignored by the server.
- Tweets sent out to room members should have the
  format `{"user":"xX_f4th3r_0f_chr1stm4s_Xx","message":"Hello North Pole!"}` where user is the author of the tweet (the
  username that the sender used in the endpoint's URL path).
- Every time a tweet is successfully sent out to a user, it counts as one view.
- Keep a running count of the number of views that happen, and return the current view count from the /19/views endpoint
  whenever requested.
- When a websocket closes, that user leaves the room and should no longer receive tweets.
- When the reset endpoint is called, the counter is set to 0.

The view counter can be in-memory and does not need to persist.

## [🎄 Day 20: Git good](src/days/day_20.rs)

Santa frowned, his usually merry eyes scanning the data on the screen before him. Something wasn't right. He pulled up
the database of gift orders worldwide, but there was a noticeable gap in the records. This was a serious issue, and
Santa knew the implications immediately - missing orders meant missing gifts and unhappy children. Just the thought of
it made his jolly cheer fade a bit, replaced by a hint of worry.

"Time for a trip down memory lane," Santa mumbled to himself as he trudged his way towards the archives. The archives
were a labyrinth of shelves, filled to the brim with old records and endless stacks of papers detailing all past
Christmases. It was a treasure trove of information that had slowly been digitized, but the older records, the ones that
were now in question, still lay tucked in the musty corners of the archive.

It wouldn't be an easy task, and even Santa knew it could take hours, maybe even days. But every child mattered, and
Santa would not rest until every record was found and every child got their rightful gift. With a deep breath, Santa
began his journey in the archives, determined to fill in the gaps and ensure a merry Christmas for all.

⭐ Task 1: Archive Analysis

To find some very old gift order records, Santa needs to dig deep into the archives. You offer to help him parse and
analyze the archive files.

Create a POST endpoint /20/archive_files that receives a tar archive file in binary format and returns the number of
files inside, and another POST endpoint /20/archive_files_size that does the same thing but instead returns the sum of
all file sizes.

💠 Example

Download the test file [northpole20231220.tar](assets/northpole20231220.tar) and use it like this:

```bash
curl -X POST http://localhost:8000/20/archive_files \
-H 'Content-Type: application/x-tar' \
--data-binary '@northpole20231220.tar'

6
```

```bash
curl -X POST http://localhost:8000/20/archive_files_size \
-H 'Content-Type: application/x-tar' \
--data-binary '@northpole20231220.tar'

1196282
```

🎁 Task 2: Git Santa his cookie back (350 bonus points)

Santa lost his cookie recently, and can't find it anymore. Good thing that everything in the north pole is logged in the
git logs! By using them, we can figure out the last one that saw it.

Add the endpoint POST /20/cookie. It will receive a tar archive just like before, but this time it contains a .git
directory with a repository structure inside. Extract the archive to somewhere on the file system (for example in a
temporary directory with the crate tempfile) and find the answer to the following instruction that Santa wrote down:

Find the name of the commit author and the commit hash of the most recent commit on the branch christmas that has a tree
that contains a file called santa.txt (in any directory) with the string COOKIE anywhere inside of it.

There are no merge commits in the repo (all commits have one parent, except the root commit).

💠 Example

Test file: [cookiejar.tar](assets/cookiejar.tar)

```bash
curl -X POST http://localhost:8000/20/cookie \
-H 'Content-Type: application/x-tar' \
--data-binary '@cookiejar.tar'

Grinch 71dfab551a1958b35b7436c54b7455dcec99a12c
```

## [🎄 Day 21: Around the Globe](src/days/day_21.rs)

Once upon a frosty night in Christmas' season, ol' Santa was tidying up his archives. With his rosy cheeks and a finer
air of mystery, he stumbled upon a pile of old, dusty tape drives. Intrigued, he gave a mighty tug and dust flew in the
air, making him sneeze in the most jolly way possible.

As he dusted them off, memories flooded back. Such mirth and jingle echoed in his mind. They were his old present
delivery logs and routes, the ones he hadn't seen in years!

⭐ Task 1: Flat Squares on a Round Sphere?

Santa found a bunch of old tape drives in the archives. Reading their contents revealed a bunch of coordinates in a
strange format encoded with ones and zeroes. He needs some help with parsing them.

Make a GET endpoint /21/coords/<binary> that takes a u64 in binary representation representing an S2 cell ID. Return the
cell's center coordinates in DMS format rounded to 3 decimals (see format below).

🔔 Tips

S2 Cells
Decimal degrees

💠 Examples

```bash
curl http://localhost:8000/21/coords/0100111110010011000110011001010101011111000010100011110001011011

83°39'54.324''N 30°37'40.584''W
```

```bash
curl http://localhost:8000/21/coords/0010000111110000011111100000111010111100000100111101111011000101

18°54'55.944''S 47°31'17.976''E
```

🎁 Task 2: Turbo-fast Country Lookup (300 bonus points)

When Santa rides his sleigh across the world, he crosses so many country borders that he sometimes forgets which country
he is in. He needs a handy little API for quickly checking where he has ended up.

Make a GET endpoint /21/country/<binary> with the same type of input as in Task 1, that returns the english name of the
country that the corresponding coordinates are in.

The input is guaranteed to represent coordinates that are within one country's borders.

```bash
curl http://localhost:8000/21/country/0010000111110000011111100000111010111100000100111101111011000101

Madagascar
```

## [🎄 Day 22: Dawn of the day before the day before the final day](src/days/day_22.rs)

When Christmas Eve rolls around, it's game time. Santa, decked out in his jolly red suit, strides over to his
fully-stacked sleigh. His eyes twinkle as he checks his list one last time on his high-tech sleigh dashboard. The
backend has been working like a charm after the recent upgrades.

With a crack of his whip and a hearty "Ho ho ho," off they go into the snowy night. The reindeers take off like an
orange space rocket, disappearing into the starry sky, on a journey to deliver joy to the world!

⭐ Task 1: Leave no gift behind!

During a last minute database migration in the gift order database, Santa noticed that a small de-sync happened. One
gift order slipped through the cracks and only ended up in one of the database replicas. Since it's already Dec 22nd,
Santa tells you we need to recover the lost record immediately. No child must be left without a gift this Christmas!

When Santa started extracting all gift order IDs from the database replicas, something got jumbled up and caused them to
print in a random order. Great... now we have two long lists of random numbers with just one number differing between
them. Santa knows you are good at coding, so he concatenates the two files, scrambles the order again and lets you find
the integer without a pair.

Make a POST endpoint /22/integers that takes in a text body with one positive u64 integer on each line. All integers
appear twice, except for one. Find it and respond with a string consisting of that number of Present emojis (🎁).

Food for thought: How memory efficient can you make your solution? Is it possible to make the integer filtering
non-allocating?

💠 Example

```bash
curl -X POST http://localhost:8000/22/integers \
-H 'Content-Type: text/plain' \
-d '888
77
888
22
77
'

🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁
```

🎁 Task 2: The Shuttle Rocket (600 bonus points)

When Santa speeds across Earth to deliver presents, he looks up to the skies and sees the Shuttle Rocket taking off to
the stars. The crew on the rocket, in their quest to visit the stars at the edge of the CCH23 galaxy, has discovered
that some magical portals have opened near every star. The portals allow instant bidirectional travel between stars.
This saves the crew a lot of flight time to the outer edge, but now they need to figure out which portals to take in
order to get to the destination.

The input is sent as text in a POST request to /22/rocket in this format:

The first line has a number N (2 <= N <= 100), the number of stars in the galaxy.
On the following N lines are the 3D coordinates of each star in the galaxy as three i32s.
Then follows a line with the number K (1 <= K <= 100), the number of portals in the galaxy.
On the following K lines are the stars that each portal connects as two star indices.
The crew wants to travel from star 0 to star N-1 on the path that goes through the least amount of portals, since going
through portals make them feel dizzy.

After the path with the least portals has been found, the crew wants to know:

How many portals did they have to go through?
How long would the path they took have been if no portals existed? (rounded to 3 decimals)
Remember to not get stuck in an infinite portal loop!

💠 Example

```bash
curl -X POST http://localhost:8000/22/rocket \
-H 'Content-Type: text/plain' \
-d '5
0 1 0
-2 2 3
3 -3 -5
1 1 5
4 3 5
4
0 1
2 4
3 4
1 2
'

3 26.123
```

Explanation:

There are 5 stars and 4 portals. We can get from star 0 to star 4 by going through these portals:

portal 0 from star 0 to star 1
portal 3 from star 1 to star 2
portal 1 from star 2 to star 4

The path is 0 -> 1 -> 2 -> 4. 3 portals were used. The length of this path without taking any portals would have been
`distance(star 0, star 1) + distance(star 1, star 2) + distance(star 2, star 4)` where `distance()` is the distance
between two stars.

