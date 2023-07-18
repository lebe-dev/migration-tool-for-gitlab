# Migration Tool for GitLab 

Migrate groups and projects from one gitlab instance to another.

## How to use

Prepare config:

```shell
cp gmt.yml-dist gmt.yml
```

Edit `gmt.yml`.

### 1. Migration

**1. Make backup**

Tool doesn't contain any API delete-calls against source GitLab instance, despite this you have to backup your data before migration process.
Migration tool provided AS IS, NO WARRANTY :)

**2. Do migration:**

```shell
./gmt migrate
```

Check `gmt.log` for migration progress and details.

### 2. Show empty projects

```shell
./gmt show-empty
```

## Limitations

- Two levels of groups are supported. Examples: `GroupName` or `Groupname/SubGroupName`..
- Target repos will have `private` visibility
- No user permissions support
- Tested in environment:
  - OS: ArchLinux (latest)
  - Source GitLab v10.x
  - Target GitLab v16.x
  - git v2.41.0
- Doesn't support retry for migration steps

## Resume process

Tool doesn't support any kind of retry mechanism for migration steps, if you want to resume interrupted process just restart the app.

Also I would recommend you to enable `error-handlers.remove-target-repo-after-clone-error` for fresh target instance (GitLab).


## Troubleshooting

Check `gmt.log` for details.