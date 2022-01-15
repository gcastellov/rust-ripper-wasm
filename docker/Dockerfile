FROM rust:1.58.0 as build_rust

# Install node
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash - \
    && apt-get install -y nodejs

FROM build_rust AS build_rust_node
WORKDIR /app
COPY ./src /app

FROM build_rust_node AS build_wasm
RUN curl -sL https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

FROM build_wasm AS build_final
WORKDIR /app/ripper_wasm
RUN wasm-pack build
WORKDIR /app/ripper_wasm/pkg
RUN npm link
WORKDIR /app/site
RUN npm link rust_ripper_wasm && npm install && npm run build-prod

FROM nginx:1.19.8-alpine
COPY ./docker/mime.types /etc/nginx/mime.types
COPY --from=build_final app/site/assets/. /usr/share/nginx/html/assets/
COPY --from=build_final app/site/js/. /usr/share/nginx/html/js/
COPY --from=build_final app/site/dist/. /usr/share/nginx/html/dist/
COPY --from=build_final app/site/index.html /usr/share/nginx/html
COPY --from=build_final app/site/cipher.html /usr/share/nginx/html
COPY --from=build_final app/site/style.css /usr/share/nginx/html
