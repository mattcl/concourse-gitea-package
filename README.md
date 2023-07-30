# (WIP) A concourse CI resource for gitea generic packages

This should be considered _very_ WIP. There is currently no support for
pagination when checking for resource versions.

## Known issues

- [ ] Does not follow pagination links when checking for resources.

## Installing

```yaml
resource_types:
  - name: gitea-package
    type: registry-image
    source:
      repository: mattcl/concourse-gitea-package
      tag: "0.2.0"
```

## Source configuration

* `uri`: *Required.* The base uri of the gitea server (e.g. `https://foo.bar.com`).
* `owner`: *Required.* The user or organization that owns the package.
* `token`: *Required.* An access token for interacting with the package registry.
* `package`: *Required.* The specific package to interact with.


### Example

Resource configuration for a gitea generic package registry.
```yaml
resources:
  - name: my-gitea-package
    type: gitea-package
    icon: package-up
    source:
      uri: "https://gitea.bar.com"
      owner: someone
      token: df8d0c1d15d37e08815d8b53cb9f6d32fd9b89ab
      package: my-package
```

## Behavior

### `check`: Check for new package versions

This will identify new versions in the gitea generic package registry. _It will
not detect new files for a specific version._

### `in`: Download file(s) associated with a given version

This will download all files associated with the given version.

### `out`: Upload file(s) for a given version

This creates the specified version in the package registry, uploading the
specified files. _Regardless of specified paths, only the basename of the file
is used._

In the event that a file with the given basename already exists for the
specified version, it is skipped. This is partially because gitea requires the
file to be deleted first, but mostly because concourse still lacks sufficient
control-flow constructs to prevent unnecessary uploads/image builds/etc.

#### Parameters

* `version`: *Required.* The version to create.
* `files`: *Required* The files to upload to the created version.

##### Example

```yaml
  - name: my-job
    plan:
      # tasks ...
      - put: my-gitea-package
        inputs:
          - gnu-release
          - musl-release
        params:
          version: "1.2.3"
          files:
            - gnu-release/file1
            - musl-release/file2

```
