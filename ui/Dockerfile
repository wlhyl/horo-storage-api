FROM node:20.18.0 as build
WORKDIR /app
COPY ./ /app/

RUN npm install --registry https://registry.npm.taobao.org
#RUN npm run build
RUN npx ng build --deploy-url /horo-admin/

RUN gzip /app/dist/horo_storage/browser/*js && gzip /app/dist/horo_storage/browser/*css #  && gzip /app/dist/horo_storage/browser/*html

FROM nginx:1.27.2-alpine

#COPY --from=build /app/dist/horo_storage/browser/index.html /usr/share/nginx/html/index.html
COPY --from=build /app/dist/horo_storage/browser /usr/share/nginx/html/horo-admin

COPY ./nginx-custom.conf /etc/nginx/conf.d/default.conf

RUN sed -i 's/worker_processes .*;/worker_processes 1;/' /etc/nginx/nginx.conf
