FROM rust:1.50.0 as build_rust

# Install node
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash - \
    && apt-get install -y nodejs

FROM build_rust AS build_rust_node

WORKDIR /app
ADD ./src /app  

FROM build_rust_node AS build_wasm

RUN cargo install wasm-pack

FROM build_wasm AS build_final
WORKDIR /app/ripper_wasm
RUN wasm-pack build
WORKDIR /app/ripper_wasm/pkg
RUN npm link
WORKDIR /app/site
RUN npm link ripper_wasm && npm install && npm run build-prod

FROM nginx:latest
COPY --from=build_final app/site/dist /usr/shared/nginx/html
COPY --from=build_final app/site/mime.types /etc/nginx/mime.types
COPY --from=build_final app/site/assets/. /usr/share/nginx/html/assets/
COPY --from=build_final app/site/dist/. /usr/share/nginx/html/dist/
COPY --from=build_final app/site/index.html /usr/share/nginx/html
COPY --from=build_final app/site/style.css /usr/share/nginx/html