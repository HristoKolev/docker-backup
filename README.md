# xdxd-backup
A backup system written in Rust.

## create

Creates a backup.

### Parameters:
`--archive-type, -t` **(required)** => The type of archive you want to create.

`--file, -f` => Where to create the archive. If not specified uses the default directory from the archive config.

`--no-encryption, -d` => If it's specified it does not encrypt the archive.

### Examples:

* Creates a docker-volumes archive.

```sh

xdxd-backup create -t docker-volumes

```
* Creates a docker-volumes archive in the `/home` directory without using encryption.

```sh

xdxd-backup create -t docker-volumes -f /home/docker-volumes.backup --no-encryption

```

## restore

Restores a backup.

### Parameters:
`--archive-type, -t` **(required)** => The type of archive you want to restore.

`--file, -f` **(required)** => Where to restore the archive from.

`--no-decryption, -d` => If it's specified it does not decrypt the archive.

### Examples:

* Restores a docker-volumes archive.

```sh

xdxd-backup restore -t docker-volumes -f /home/docker-volumes.backup

```

## unpack

Unpacks a backup into a specified directory.

### Parameters:
`--archive-type, -t` **(required)** => The type of archive you want to unpack.

`--file, -f` **(required)** => Where to unpack the archive from.

`--out-path, -o` **(required)** => Where to unpack the archive to.

`--no-decryption, -d` => If it's specified it does not decrypt the archive.

### Examples:

* Unpacks a docker-volumes archive.

```sh

xdxd-backup unpack -t docker-volumes -f /home/docker-volumes.backup -o /home/unpacked

```

## list

Prints information about all the locally stored archives.

### Parameters:

`--archive-type, -t` => Optional filtering by archive type.

## clear-cache

Clears the local archive cache.

### Parameters:

`--archive-type, -t` => Optional filtering by archive type.

## clear-remote-cache

Clears the remote archive cache.

### Parameters:

`--archive-type, -t` => Optional filtering by archive type.

## upload

Uploads any archives that are present in the local cache but are not present in any of the remote caches.

### Parameters:

`--archive-type, -t` => Optional filtering by archive type.

## config

Prints the config that the application uses. 
