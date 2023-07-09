
# Url shortener back Rust implementation

## Used crates 

- [actix_web](https://actix.rs/)

## Deploy

### Prerequisites

- We assume that the repository is cloned into `~/url_shortener`

- `~/.ssh/config` contains something like that:

```
Host u2h
HostName u2h.ru
User z9v
IdentityFile ~/it-management/users/zv/id_rsa
```

### Deploy to `dev`

```bash
~/url_shortener/di/dev/url_shortener/nginx.sh
~/url_shortener/di/dev/url_shortener/systemd.log.sh
~/url_shortener/di/dev/url_shortener/systemd.sh
~/url_shortener/di/dev/url_shortener/deploy.sh
```

#### After deploy to `dev` following REST API is availalbe

```bash
curl 'https://u2h.ru/dev/url_shortener_back/about' 
curl -w "\n" -X POST 'https://u2h.ru/dev/url_shortener_back/shorten' -H 'Content-Type: application/json' -d '{ "url": "https://github.com/yurybikuzin/url_shortener" }' 
curl -i -w "\n" 'https://u2h.ru/dev/N:FY5' 
curl -w "\n" 'https://u2h.ru/dev/url_shortener_back/stat/N:FY5' 
```

- [https://u2h.ru/dev/url_shortener_back/about](https://u2h.ru/dev/url_shortener_back/about): should report **op_mode**(`prod`/`dev`/`demo`/`rc`), **app_name** and **version**

- [https://u2h.ru/dev/N:FY5](https://u2h.ru/dev/N:FY5): should redirect to `https://github.com/yurybikuzin/url_shortener`

- [https://u2h.ru/dev/url_shortener_back/stat/N:FY5](https://u2h.ru/dev/url_shortener_back/stat/N:FY5): should provide stat of redirects


### Deploy to `prod`

```bash
~/url_shortener/di/prod/url_shortener/nginx.sh
~/url_shortener/di/prod/url_shortener/systemd.log.sh
~/url_shortener/di/prod/url_shortener/systemd.sh
~/url_shortener/di/prod/url_shortener/deploy.sh
```

#### After deploy to `prod` following REST API is availalbe

```bash
curl 'https://u2h.ru/url_shortener_back/about' 
curl -w "\n" -X POST 'https://u2h.ru/url_shortener_back/shorten' -H 'Content-Type: application/json' -d '{ "url": "https://github.com/yurybikuzin/url_shortener" }' 
curl -i -w "\n" 'https://u2h.ru/N:FY5' 
curl -w "\n" 'https://u2h.ru/url_shortener_back/stat/N:FY5' 
```

- [https://u2h.ru/url_shortener_back/about](https://u2h.ru/url_shortener_back/about): should report **op_mode**(`prod`/`dev`/`demo`/`rc`), **app_name** and **version**

- [https://u2h.ru/N:FY5](https://u2h.ru/N:FY5): should redirect to `https://github.com/yurybikuzin/url_shortener`

- [https://u2h.ru/url_shortener_back/stat/N:FY5](https://u2h.ru/url_shortener_back/stat/N:FY5): should provide stat of redirects

