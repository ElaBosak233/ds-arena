# ds-arena

A simple validator for Data Security challenge.

## How to use

Prepare a CSV file named `criteria.csv`, then prepare a Dockerfile like this:

```dockerfile
FROM elabosak233/ds-arena:latest

COPY ./criteria.csv /app/resources/criteria.csv

ENV FLAG=cdsctf{whats_your_prolem}
```

Then build and run your own image.

The port is `8889`.