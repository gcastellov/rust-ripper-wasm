FROM nginx
COPY mime.types /etc/nginx/mime.types
COPY /assets/. /usr/share/nginx/html/assets/
COPY /dist/. /usr/share/nginx/html/dist/
COPY index.html /usr/share/nginx/html
COPY style.css /usr/share/nginx/html