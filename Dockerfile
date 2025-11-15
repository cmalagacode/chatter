FROM ubuntu:latest
LABEL authors="cmalaga"

ENTRYPOINT ["top", "-b"]