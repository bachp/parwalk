# Parallel Walker (Parwalk)

Parwalk visits a directory tree recursively. It can optionally read the files content.

The main purpose is to do file system scrubing as well as [initializing AWS EBS volumes](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-initialize.html).

The recursive descent is done in parallel on a worker pool using a breadth first approach.

## Usage

The following command will  walk the `/data` directory recursively.

```
parwalk /data
```

By default Parwalk will only visit directory.

It is also possbile to read the content of each visited file.

```
parwalk -r /data
```

This is useful for scrubing the whole content of directory.

### Enable logging

By default Parwalk will only log errors.

Logging can be controlled via [`env_logger`](https://docs.rs/env_logger) trough the
`RUST_LOG` environment variable.

```
RUST_LOG=info parwalk /data
```

Will log inforatmion about all visited directories nut just errors.

### Control concurrency

By default Parwalk will use as many workers as there are cores on the machine.

The concurrency is powered by [`rayon`](https://docs.rs/rayon/).

In order to change the number of working threads the `RAYON_NUM_THREADS` environment
variable can be used.

```
RAYON_NUM_THREADS=64 parwalk /data
```

Will run parwalk with 64 worker threads.
