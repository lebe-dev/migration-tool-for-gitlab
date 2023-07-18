# Migration Tool for GitLab 

Migrate groups and projects from one gitlab instance to another.

## How to use

**1. Make backup**

Tool doesn't contain any API delete-calls against GitLab instances, despite this you have to backup your data before migration process.
Migration tool provided AS IS, NO WARRANTY :)

**2. Prepare config:**

```shell
cp gmt.yml-dist gmt.yml
```

Edit `gmt.yml`.

**3. Do migration:**

```shell
./gmt migrate
```

## Limitations

- Two levels of groups. Examples: `GroupName` or `Groupname/SubGroupName`. No deeper levels are supported.
- Target repos will have `private` visibility
- No user permissions support
- Tool tested with environment:
  - OS: ArchLinux (latest)
  - Source GitLab v10.x
  - Target GitLab v16.x
  - git v2.41.0

## Troubleshooting

Check `gmt.log` for details.