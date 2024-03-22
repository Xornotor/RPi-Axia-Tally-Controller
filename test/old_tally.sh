#!/bin/bash
curl -X POST 127.0.0.1:9000/reconfig -H 'Content-Type: application/json' -d @old_tally.json