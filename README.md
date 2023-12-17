Shuttle's Christmas Code Hunt
=============================

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

## 🎄 Day 5 got missing, IDK the reason

The Grinch stole the contents of this challenge while the Shuttle team wasn't looking! Check back soon in case we find
the challenge contents!

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
