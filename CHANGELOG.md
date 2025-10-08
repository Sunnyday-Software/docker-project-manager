## [1.6.2](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.6.1...v1.6.2) (2025-10-08)


### Bug Fixes

* input arguments ([6e7cdb5](https://github.com/Sunnyday-Software/docker-project-manager/commit/6e7cdb5f1f6fafb0cf0634c683a24a05eccc7d37))

## [1.6.1](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.6.0...v1.6.1) (2025-10-07)


### Bug Fixes

* typos ([38f1115](https://github.com/Sunnyday-Software/docker-project-manager/commit/38f11151eee214b333ce8bba1b02fab4e74edabc))

# [1.6.0](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.5.0...v1.6.0) (2025-10-07)


### Features

* reworked tty logic and input arguments ([816b464](https://github.com/Sunnyday-Software/docker-project-manager/commit/816b4649763163f94aa40eebdb45fbadb9718c33)), closes [#0](https://github.com/Sunnyday-Software/docker-project-manager/issues/0)

# [1.5.0](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.4.1...v1.5.0) (2025-09-16)


### Features

* hide sensitive env vars ([371a312](https://github.com/Sunnyday-Software/docker-project-manager/commit/371a312a14c7f13ae0f858a06af06251a0809dff))

## [1.4.1](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.4.0...v1.4.1) (2025-09-14)


### Bug Fixes

* regex deps ([77ea098](https://github.com/Sunnyday-Software/docker-project-manager/commit/77ea09898389582ea62f4013218379ebfde7b85e))

# [1.4.0](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.3.0...v1.4.0) (2025-09-14)


### Features

* file list ([0d911ce](https://github.com/Sunnyday-Software/docker-project-manager/commit/0d911cec9272f77b3a5353ab9ed1369529e57184))

# [1.3.0](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.2.0...v1.3.0) (2025-07-04)


### Features

* **file_ops:** include relative file paths in MD5 hashing ([dc5b1c6](https://github.com/Sunnyday-Software/docker-project-manager/commit/dc5b1c62e9c1f6baa9b3d65b5455032a5b0308f2))

# [1.2.0](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.1.7...v1.2.0) (2025-07-03)


### Features

* enhance S-expression parsing and add Docker CLI configuration commands ([78eeb5c](https://github.com/Sunnyday-Software/docker-project-manager/commit/78eeb5c099ec183252b0b37bff51c3651e223dc5))

## [1.1.7](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.1.6...v1.1.7) (2025-07-02)


### Bug Fixes

* **ci:** test ci ([24c5b69](https://github.com/Sunnyday-Software/docker-project-manager/commit/24c5b6914c168432e1862f9f64ba659492751543))

## [1.1.6](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.1.5...v1.1.6) (2025-07-02)


### Bug Fixes

* **ci:** test ci ([cbce4fc](https://github.com/Sunnyday-Software/docker-project-manager/commit/cbce4fcc188cfecff5769c85d8e3207f84880770))
* **ci:** test ci ([4c3f941](https://github.com/Sunnyday-Software/docker-project-manager/commit/4c3f94144855cb0418694d4c7fb4969b5fffa3e6))

## [1.1.5](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.1.4...v1.1.5) (2025-07-02)


### Bug Fixes

* **ci:** pass inputs to `build-multiarch` workflow and update artifact naming ([e5d8ad7](https://github.com/Sunnyday-Software/docker-project-manager/commit/e5d8ad7e4f325f1039d00b8367e758d199d24bdb))

## [1.1.4](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.1.3...v1.1.4) (2025-07-02)


### Bug Fixes

* **ci:** consolidate OS-specific workflows into a single matrix-based multi-arch workflow ([e27ddd0](https://github.com/Sunnyday-Software/docker-project-manager/commit/e27ddd02469e90872dedb8226e8a1c6809fad0a2))

## [1.1.3](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.1.2...v1.1.3) (2025-07-02)


### Bug Fixes

* **ci:** remove unnecessary conditions and dependencies in release workflow ([0939c4e](https://github.com/Sunnyday-Software/docker-project-manager/commit/0939c4ec64d36bad356b0840328cba33c983371e))
* **ci:** simplify workflows with `workflow_call` trigger and consolidate actions ([1b3dc1c](https://github.com/Sunnyday-Software/docker-project-manager/commit/1b3dc1c51a3b8b00b8a2dd202c2552e1ab87fcb2))

## [1.1.2](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.1.1...v1.1.2) (2025-07-02)


### Bug Fixes

* **ci:** replace custom ARM64 image with prebuilt sunnydaysoftware image for faster builds ([07e51dd](https://github.com/Sunnyday-Software/docker-project-manager/commit/07e51dd2f6d20fef0b5c649fc01a3af435c425dc))

## [1.1.1](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.1.0...v1.1.1) (2025-07-02)


### Bug Fixes

* **ci:** replace OS matrix build with multi-arch workflows triggered on dependent workflow completion ([b8abcd3](https://github.com/Sunnyday-Software/docker-project-manager/commit/b8abcd35456510027504a393697530b056b88c39))

# [1.1.0](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.0.2...v1.1.0) (2025-07-02)


### Features

* **ci:** add multi-architecture build and test workflows ([6b5ba7b](https://github.com/Sunnyday-Software/docker-project-manager/commit/6b5ba7b3f6d55b4520f4ea697118979023f3fff6))

## [1.0.2](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.0.1...v1.0.2) (2025-07-02)


### Bug Fixes

* **install:** correct typo ([0d0ccbd](https://github.com/Sunnyday-Software/docker-project-manager/commit/0d0ccbddba2b8203621377f62bcceb2fd9737854))

## [1.0.1](https://github.com/Sunnyday-Software/docker-project-manager/compare/v1.0.0...v1.0.1) (2025-06-30)


### Bug Fixes

* version number ([1c7629c](https://github.com/Sunnyday-Software/docker-project-manager/commit/1c7629c14dd025785a43fab52eeb443f8c9b3051))
