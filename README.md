# xdxd-backup
A backup system written in Rust.

## create
Creates a backup.

### Parameters:
`--archive-type, -t` **(required)** => The type of archive you want to create.
`--file, -f` => Where to create the archive. If not specified uses the default directory from the archive config.
`--no-encryption, -d` => If it's specified it does not encrypt the archives.

### Examples:

* Creates a docker-volumes archive.

```sh
xdxd-backup create -t docker-volumes
```
* Creates a docker-volumes archive in the `/home` directory without using encryption.

```sh
xdxd-backup create -t docker-volumes -f /home/docker-volumes.backup --no-encryption
```

