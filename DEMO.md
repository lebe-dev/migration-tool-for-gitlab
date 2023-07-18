# Demo environment

If you want to try tool before real migration, you can spin up a demo environment. Data will be copied from to gitlab demo instance. 
This document describes how to do it.

**1. Install docker and docker-compose**

**2. Prepare directory for demo GitLab**

```shell
mkdir /opt/gitlab
```

Create `/opt/gitlab/docker-compose.yml` with content:

```yaml
version: '3.6'

services:
  web:
    image: 'gitlab/gitlab-ce:16.0.7-ce.0'
    restart: always
    environment:
      GITLAB_OMNIBUS_CONFIG: |
        gitlab_rails['gitlab_shell_ssh_port'] = 2222
        letsencrypt['enable'] = false

        nginx['enable'] = true
        nginx['listen_https'] = false
        nginx['redirect_http_to_https'] = false
    ports:
      - '28080:80'
      - '28443:443'
      - '2222:22'
    volumes:
      - './config:/etc/gitlab'
      - './logs:/var/log/gitlab'
      - './data:/var/opt/gitlab'
    shm_size: '256m'
```

Then

```shell
docker-compose up -d
```

After few minutes GitLab will be available at http://localhost:28080

Get root password:

```shell
docker exec -it gitlab-web-1 cat /etc/gitlab/initial_root_password
```

Then login with `root` and obtained password.

**3. Get api token from source GitLab instance**

Go to source GitLab instance (where you want to migrate your data). User Settings > Access tokens. Create a new token with
all options. Copy token value somewhere it will be useful later.

**4. Get api token from target GitLab instance**

Go to target GitLab instance at http://localhost:28080 then User Settings > Access tokens. Create a new token with
all options. Copy token value somewhere.

**4. Prepare migration tool**

```shell
cp gmt.yml-dist gmt.yml
```

Edit `gmt.yml`, paste obtained api tokens:

```yml
...

source:
  public-url: 'https://source-gitlab-address'
  git-url: 'ssh://git@source-gitlab-address:22'
  token: 'PAST-FIRST-API-TOKEN-HERE'

target:
  public-url: 'http://localhost:28080'
  git-url: 'ssh://git@localhost:2222'
  token: 'PAST-SECOND-API-TOKEN-HERE'
```


**5. Start migration**

```shell
chmod +x gmt
./gmt migrate
```

Check `gmt.log` for migration progress.