FROM node:23.3.0-alpine3.19 AS build
ARG VITE_MAPS_API_KEY

WORKDIR /app
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable && npm install -g corepack@latest

COPY . /app

RUN pnpm install

ENV PATH /app/node_modules/.bin:$PATH

COPY . .

RUN pnpm run build

FROM nginx:1.25.4-alpine3.18

COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /var/www/html/

EXPOSE 3000

ENTRYPOINT ["nginx","-g","daemon off;"]
