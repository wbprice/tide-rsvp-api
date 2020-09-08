FROM rust:1.46
WORKDIR /usr/src/rsvp
COPY . .

RUN cargo install --path .

CMD ["test-tide"]
