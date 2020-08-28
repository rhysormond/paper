FROM rust:latest as rust
WORKDIR src
# cache wasm-related dependencies
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack wasm-bindgen-cli
# cache application dependencies (add an empty lib file to make cargo happy)
COPY Cargo.toml .
RUN mkdir src && touch src/lib.rs
RUN cargo fetch
# build the library
COPY src ./src
RUN wasm-pack build

FROM node:latest
WORKDIR src
# cache application dependencies
COPY www/package.json www/package.json
RUN npm install --loglevel=error
# copy the wasm dependency
COPY --from=rust src/pkg ./pkg
# copy the application code
COPY www ./www
# run the application
WORKDIR www
CMD ["npm", "run", "start"]
