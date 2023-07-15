window.BENCHMARK_DATA = {
  "lastUpdate": 1689444666577,
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
      }
    ]
  }
}