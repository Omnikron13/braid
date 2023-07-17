window.BENCHMARK_DATA = {
  "lastUpdate": 1689601120733,
  "repoUrl": "https://github.com/Omnikron13/braid",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "committer": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "distinct": true,
          "id": "6f44d0b713ad8b90928c7012eea3b254626e622b",
          "message": "condensed benches/remove.rs",
          "timestamp": "2023-07-15T18:54:42+01:00",
          "tree_id": "2468a1abb3b88d515556f7366810aed5f7665bf3",
          "url": "https://github.com/Omnikron13/braid/commit/6f44d0b713ad8b90928c7012eea3b254626e622b"
        },
        "date": 1689444665732,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str//small",
            "value": 7858,
            "range": "± 906",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//medium",
            "value": 1079179,
            "range": "± 11546",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//large",
            "value": 7524957,
            "range": "± 69113",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//linefeeds",
            "value": 234745,
            "range": "± 276",
            "unit": "ns/iter"
          },
          {
            "name": "rope_clone",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/small",
            "value": 104,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/small",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/small",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/medium",
            "value": 14239,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/medium",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/medium",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/large",
            "value": 101451,
            "range": "± 379",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/large",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/large",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/unicode_01",
            "value": 212,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/unicode_01",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/unicode_01",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/cyrillic_01",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/cyrillic_01",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/cyrillic_01",
            "value": 103,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/tiny",
            "value": 301,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/tiny",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/small",
            "value": 644,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/small",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/medium",
            "value": 87984,
            "range": "± 6814",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/medium",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/large",
            "value": 623971,
            "range": "± 122279",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/large",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/unicode_01",
            "value": 1292,
            "range": "± 170",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/unicode_01",
            "value": 69,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/cyrillic_01",
            "value": 590,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/cyrillic_01",
            "value": 267,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/1024",
            "value": 4545,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/1024",
            "value": 3366,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/1024",
            "value": 5433,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/1024",
            "value": 5116,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/4096",
            "value": 17819,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/4096",
            "value": 13390,
            "range": "± 350",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/4096",
            "value": 20409,
            "range": "± 366",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/4096",
            "value": 18236,
            "range": "± 258",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/small",
            "value": 1410,
            "range": "± 3546",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/small",
            "value": 533,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/small",
            "value": 1500,
            "range": "± 50",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "committer": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "distinct": true,
          "id": "5046aa3fd9e6b8edceab703424e32f03da07cbd8",
          "message": "indent",
          "timestamp": "2023-07-15T19:17:52+01:00",
          "tree_id": "7ebd17c090f21c697d549aabe579969db1b12aee",
          "url": "https://github.com/Omnikron13/braid/commit/5046aa3fd9e6b8edceab703424e32f03da07cbd8"
        },
        "date": 1689446351963,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str//small",
            "value": 7731,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//medium",
            "value": 1056219,
            "range": "± 1250",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//large",
            "value": 7386836,
            "range": "± 6960",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//linefeeds",
            "value": 179706,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "rope_clone",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/small",
            "value": 97,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/small",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/small",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/medium",
            "value": 13088,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/medium",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/medium",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/large",
            "value": 96245,
            "range": "± 1714",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/large",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/large",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/unicode_01",
            "value": 195,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/unicode_01",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/unicode_01",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/cyrillic_01",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/cyrillic_01",
            "value": 59,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/cyrillic_01",
            "value": 90,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/tiny",
            "value": 481,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/tiny",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/small",
            "value": 1052,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/small",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/medium",
            "value": 147458,
            "range": "± 14470",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/medium",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/large",
            "value": 1026158,
            "range": "± 273911",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/large",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/unicode_01",
            "value": 2084,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/unicode_01",
            "value": 67,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/cyrillic_01",
            "value": 962,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/cyrillic_01",
            "value": 239,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/1024",
            "value": 4985,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/1024",
            "value": 3467,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/1024",
            "value": 6502,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/1024",
            "value": 6083,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/4096",
            "value": 18926,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/4096",
            "value": 13788,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/4096",
            "value": 23595,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/4096",
            "value": 22344,
            "range": "± 294",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/small",
            "value": 1431,
            "range": "± 744",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/small",
            "value": 538,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/small",
            "value": 1555,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/small",
            "value": 568,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/medium",
            "value": 1736,
            "range": "± 609",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/medium",
            "value": 622,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/medium",
            "value": 2877,
            "range": "± 117",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/medium",
            "value": 633,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/large",
            "value": 8543,
            "range": "± 1891",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/large",
            "value": 7507,
            "range": "± 147",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/large",
            "value": 10800,
            "range": "± 351",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/large",
            "value": 7528,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "insert_after_clone",
            "value": 14676,
            "range": "± 278",
            "unit": "ns/iter"
          },
          {
            "name": "remove/random/small",
            "value": 1297,
            "range": "± 153",
            "unit": "ns/iter"
          },
          {
            "name": "remove/start/small",
            "value": 4193,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "remove/middle/small",
            "value": 6388,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "remove/end/small",
            "value": 19258,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "remove/random/medium",
            "value": 1554,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "remove/start/medium",
            "value": 4213,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "remove/middle/medium",
            "value": 12488,
            "range": "± 238",
            "unit": "ns/iter"
          },
          {
            "name": "remove/end/medium",
            "value": 18950,
            "range": "± 188",
            "unit": "ns/iter"
          },
          {
            "name": "remove/random/large",
            "value": 6788,
            "range": "± 2947",
            "unit": "ns/iter"
          },
          {
            "name": "remove/start/large",
            "value": 29951,
            "range": "± 521",
            "unit": "ns/iter"
          },
          {
            "name": "remove/middle/large",
            "value": 50027,
            "range": "± 4793",
            "unit": "ns/iter"
          },
          {
            "name": "remove/end/large",
            "value": 76375,
            "range": "± 7326",
            "unit": "ns/iter"
          },
          {
            "name": "remove_initial_after_clone",
            "value": 2483,
            "range": "± 37",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "committer": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "distinct": true,
          "id": "d218ce4f87df434a6d28fd1989413df531d24b6e",
          "message": "switch from Rc to Arc - potential benefits for sync shit outweighs essentially non-existent performance penalty",
          "timestamp": "2023-07-15T20:18:09+01:00",
          "tree_id": "adfaa022bbf741e9d938b17b86ece43e9f7510a1",
          "url": "https://github.com/Omnikron13/braid/commit/d218ce4f87df434a6d28fd1989413df531d24b6e"
        },
        "date": 1689449990336,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str//small",
            "value": 7737,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//medium",
            "value": 1056670,
            "range": "± 1553",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//large",
            "value": 7394776,
            "range": "± 11133",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//linefeeds",
            "value": 183051,
            "range": "± 423",
            "unit": "ns/iter"
          },
          {
            "name": "rope_clone",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/small",
            "value": 97,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/small",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/small",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/medium",
            "value": 13103,
            "range": "± 383",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/medium",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/medium",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/large",
            "value": 95891,
            "range": "± 210",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/large",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/large",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/unicode_01",
            "value": 195,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/unicode_01",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/unicode_01",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/cyrillic_01",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/cyrillic_01",
            "value": 59,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/cyrillic_01",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/tiny",
            "value": 481,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/tiny",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/small",
            "value": 1052,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/small",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/medium",
            "value": 147147,
            "range": "± 13863",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/medium",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/large",
            "value": 1018902,
            "range": "± 278490",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/large",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/unicode_01",
            "value": 2086,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/unicode_01",
            "value": 67,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/cyrillic_01",
            "value": 956,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/cyrillic_01",
            "value": 239,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/1024",
            "value": 5077,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/1024",
            "value": 3461,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/1024",
            "value": 6525,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/1024",
            "value": 6307,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/4096",
            "value": 19327,
            "range": "± 196",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/4096",
            "value": 13785,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/4096",
            "value": 23743,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/4096",
            "value": 22959,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/small",
            "value": 1647,
            "range": "± 1458",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/small",
            "value": 743,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/small",
            "value": 2343,
            "range": "± 228",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/small",
            "value": 718,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/medium",
            "value": 1959,
            "range": "± 1113",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/medium",
            "value": 868,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/medium",
            "value": 4837,
            "range": "± 668",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/medium",
            "value": 831,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/large",
            "value": 9317,
            "range": "± 2249",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/large",
            "value": 8200,
            "range": "± 304",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/large",
            "value": 13128,
            "range": "± 946",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/large",
            "value": 9866,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "insert_after_clone",
            "value": 14169,
            "range": "± 170",
            "unit": "ns/iter"
          },
          {
            "name": "remove/random/small",
            "value": 1547,
            "range": "± 199",
            "unit": "ns/iter"
          },
          {
            "name": "remove/start/small",
            "value": 4964,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "remove/middle/small",
            "value": 6395,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "remove/end/small",
            "value": 19960,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "remove/random/medium",
            "value": 1830,
            "range": "± 250",
            "unit": "ns/iter"
          },
          {
            "name": "remove/start/medium",
            "value": 4989,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "remove/middle/medium",
            "value": 12445,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "remove/end/medium",
            "value": 19774,
            "range": "± 210",
            "unit": "ns/iter"
          },
          {
            "name": "remove/random/large",
            "value": 6978,
            "range": "± 3011",
            "unit": "ns/iter"
          },
          {
            "name": "remove/start/large",
            "value": 32117,
            "range": "± 536",
            "unit": "ns/iter"
          },
          {
            "name": "remove/middle/large",
            "value": 51221,
            "range": "± 5315",
            "unit": "ns/iter"
          },
          {
            "name": "remove/end/large",
            "value": 76566,
            "range": "± 7671",
            "unit": "ns/iter"
          },
          {
            "name": "remove_initial_after_clone",
            "value": 2514,
            "range": "± 38",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "committer": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "distinct": true,
          "id": "c55549efb3ee97c452ea46fc2c9c9df6c5de04b9",
          "message": "add newline_iter()/newline_index_iter() method(s)",
          "timestamp": "2023-07-16T00:18:28+01:00",
          "tree_id": "c542372febcbc30c87cccccb266d3523db712537",
          "url": "https://github.com/Omnikron13/braid/commit/c55549efb3ee97c452ea46fc2c9c9df6c5de04b9"
        },
        "date": 1689464028458,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str//small",
            "value": 6203,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//medium",
            "value": 836458,
            "range": "± 1122",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//large",
            "value": 5855374,
            "range": "± 6922",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//linefeeds",
            "value": 84853,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "rope_clone",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/small",
            "value": 120,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/medium",
            "value": 16356,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/large",
            "value": 115139,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/unicode_01",
            "value": 243,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/cyrillic_01",
            "value": 105,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/tiny",
            "value": 477,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/tiny",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/small",
            "value": 1047,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/small",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/medium",
            "value": 146122,
            "range": "± 14784",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/medium",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/large",
            "value": 1037733,
            "range": "± 268264",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/large",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/unicode_01",
            "value": 2078,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/unicode_01",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/cyrillic_01",
            "value": 1030,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/cyrillic_01",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/1024",
            "value": 5255,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/1024",
            "value": 3808,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/1024",
            "value": 6068,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/1024",
            "value": 5652,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/4096",
            "value": 19884,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/4096",
            "value": 15129,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/4096",
            "value": 22133,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/4096",
            "value": 20071,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/small",
            "value": 1606,
            "range": "± 932",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/small",
            "value": 637,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/small",
            "value": 1856,
            "range": "± 115",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/small",
            "value": 631,
            "range": "± 21",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "committer": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "distinct": false,
          "id": "b5777adc094b3889b0a75cbcf21e0c306b2e5ef9",
          "message": "alias std::ops::",
          "timestamp": "2023-07-16T04:24:55+01:00",
          "tree_id": "cd04c17692ae08b462708707ac80451e1a6afc1b",
          "url": "https://github.com/Omnikron13/braid/commit/b5777adc094b3889b0a75cbcf21e0c306b2e5ef9"
        },
        "date": 1689479576148,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str//small",
            "value": 5679,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//medium",
            "value": 762751,
            "range": "± 781",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//large",
            "value": 5339058,
            "range": "± 8831",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//linefeeds",
            "value": 79327,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "rope_clone",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/small",
            "value": 120,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/medium",
            "value": 16361,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/large",
            "value": 115294,
            "range": "± 218",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/unicode_01",
            "value": 242,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/cyrillic_01",
            "value": 104,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/tiny",
            "value": 479,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/tiny",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/small",
            "value": 1051,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/small",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/medium",
            "value": 146032,
            "range": "± 13890",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/medium",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/large",
            "value": 1021958,
            "range": "± 262782",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/large",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/unicode_01",
            "value": 2079,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/unicode_01",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/cyrillic_01",
            "value": 941,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/cyrillic_01",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/1024",
            "value": 5132,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/1024",
            "value": 3803,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/1024",
            "value": 5975,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/1024",
            "value": 5701,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/4096",
            "value": 20040,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/4096",
            "value": 15140,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/4096",
            "value": 22429,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/4096",
            "value": 20308,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/small",
            "value": 1639,
            "range": "± 1043",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/small",
            "value": 637,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/small",
            "value": 2193,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/small",
            "value": 637,
            "range": "± 29",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "committer": {
            "email": "joey.sabey@gmx.com",
            "name": "Joey Sabey",
            "username": "Omnikron13"
          },
          "distinct": true,
          "id": "d1bf79dacd9515d7a306c917214ba35bc7fcbb31",
          "message": "import Ranged in benches",
          "timestamp": "2023-07-17T14:14:43+01:00",
          "tree_id": "f91c56c19fb378795f9a271f183d6726f61d14bf",
          "url": "https://github.com/Omnikron13/braid/commit/d1bf79dacd9515d7a306c917214ba35bc7fcbb31"
        },
        "date": 1689601119889,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str//small",
            "value": 5833,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//medium",
            "value": 762692,
            "range": "± 706",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//large",
            "value": 5337502,
            "range": "± 6998",
            "unit": "ns/iter"
          },
          {
            "name": "from_str//linefeeds",
            "value": 79618,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "rope_clone",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/small",
            "value": 118,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/small",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/medium",
            "value": 16348,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/medium",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/large",
            "value": 115115,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/large",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/unicode_01",
            "value": 242,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/unicode_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/manual/cyrillic_01",
            "value": 102,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/chars/indexed/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/manual/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "count/bytes/indexed/cyrillic_01",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/tiny",
            "value": 480,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/tiny",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/small",
            "value": 1051,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/small",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/medium",
            "value": 147351,
            "range": "± 14072",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/medium",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/large",
            "value": 1024617,
            "range": "± 252854",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/large",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/unicode_01",
            "value": 2078,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/unicode_01",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/manual/cyrillic_01",
            "value": 940,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "byte_index/indexed/cyrillic_01",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/1024",
            "value": 5206,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/1024",
            "value": 3800,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/1024",
            "value": 6119,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/1024",
            "value": 5783,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "push/rand/4096",
            "value": 19629,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "push/uniform/4096",
            "value": 15126,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "push/alternating/4096",
            "value": 22326,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "push/cyrillic/4096",
            "value": 20414,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/small",
            "value": 1606,
            "range": "± 861",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/small",
            "value": 650,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/small",
            "value": 1762,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/small",
            "value": 625,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/medium",
            "value": 1918,
            "range": "± 678",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/medium",
            "value": 706,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/medium",
            "value": 3062,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/medium",
            "value": 700,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "insert/random/large",
            "value": 8500,
            "range": "± 1767",
            "unit": "ns/iter"
          },
          {
            "name": "insert/start/large",
            "value": 7166,
            "range": "± 169",
            "unit": "ns/iter"
          },
          {
            "name": "insert/middle/large",
            "value": 9443,
            "range": "± 142",
            "unit": "ns/iter"
          },
          {
            "name": "insert/end/large",
            "value": 7141,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "insert_after_clone",
            "value": 14899,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "remove/random/small",
            "value": 1115,
            "range": "± 650",
            "unit": "ns/iter"
          },
          {
            "name": "remove/start/small",
            "value": 4968,
            "range": "± 192",
            "unit": "ns/iter"
          },
          {
            "name": "remove/middle/small",
            "value": 6112,
            "range": "± 253",
            "unit": "ns/iter"
          },
          {
            "name": "remove/end/small",
            "value": 19741,
            "range": "± 357",
            "unit": "ns/iter"
          },
          {
            "name": "remove/random/medium",
            "value": 1276,
            "range": "± 595",
            "unit": "ns/iter"
          },
          {
            "name": "remove/start/medium",
            "value": 4348,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "remove/middle/medium",
            "value": 12636,
            "range": "± 308",
            "unit": "ns/iter"
          },
          {
            "name": "remove/end/medium",
            "value": 19320,
            "range": "± 329",
            "unit": "ns/iter"
          },
          {
            "name": "remove/random/large",
            "value": 1564,
            "range": "± 4029",
            "unit": "ns/iter"
          },
          {
            "name": "remove/start/large",
            "value": 31350,
            "range": "± 147",
            "unit": "ns/iter"
          },
          {
            "name": "remove/middle/large",
            "value": 59211,
            "range": "± 190",
            "unit": "ns/iter"
          },
          {
            "name": "remove/end/large",
            "value": 87720,
            "range": "± 230",
            "unit": "ns/iter"
          },
          {
            "name": "remove_initial_after_clone",
            "value": 2462,
            "range": "± 41",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}