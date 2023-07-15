window.BENCHMARK_DATA = {
  "lastUpdate": 1689446352533,
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
      }
    ]
  }
}