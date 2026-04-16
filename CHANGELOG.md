# Changelog

## 3.0.1 (2026-04-16)

### Bug Fixes

- **ci**: migrate sr v4 to v7 for artifact and input support (#7) ([5765d8f](https://github.com/urmzd/gymnasia/commit/5765d8f1cd472a7dc4ee5956fe53d2d2c2ff27fd))

### Misc

- migrate sr config and action to v4 ([292e565](https://github.com/urmzd/gymnasia/commit/292e565c6b7106dac355f1c67ac5dfc4acda81c4))
- commit Cargo.lock to fix release pipeline ([061b27d](https://github.com/urmzd/gymnasia/commit/061b27de7d5817a5913e9d88d36ca10555acbf95))

[Full Changelog](https://github.com/urmzd/gymnasia/compare/v3.0.0...v3.0.1)


## 3.0.0 (2026-04-09)

### Breaking Changes

- **spaces**: make Bounded::clamp abstract with Vec<f64> impl ([d525e5a](https://github.com/urmzd/gymnasia/commit/d525e5a209646bce55da92362cd1c28d13887bd8))
- **envs**: change action type from usize to i64 ([27db123](https://github.com/urmzd/gymnasia/commit/27db123937bc5e959abf2631c9cd89a1a439f204))
- v3.0.0 architecture redesign ([5ef1cd3](https://github.com/urmzd/gymnasia/commit/5ef1cd325a114f1ee7fa27e735fd2c052f597b95))

### Features

- **wrappers**: implement RescaleAction rescaling ([7792803](https://github.com/urmzd/gymnasia/commit/7792803754c47647687417890be7b44e4c713ade))
- **wrappers**: cache observation space in FlattenObservation ([120c4b7](https://github.com/urmzd/gymnasia/commit/120c4b7d99f4efe3cde13d87911c69e793252de7))
- **spaces**: add clamp method to Bounded trait and implementations ([6a0bd0d](https://github.com/urmzd/gymnasia/commit/6a0bd0d158d9c3fc09fba7db2edc92ecf2be9941))

### Bug Fixes

- address PR review findings ([21512f0](https://github.com/urmzd/gymnasia/commit/21512f02c1fc74607e595543530db6da45683999))
- **ci**: remove --allow-dirty from cargo publish ([3147851](https://github.com/urmzd/gymnasia/commit/3147851095e86f6897373fcd696dac9627ddfaa9))
- **wrappers**: normalize reward after episode reset logic ([b6a7156](https://github.com/urmzd/gymnasia/commit/b6a7156dab43f6bad682ba0eaa822429b8154ea5))

### Documentation

- **readme**: add clamp method to Bounded trait example ([2fa15ac](https://github.com/urmzd/gymnasia/commit/2fa15acb560ae61653003dda8cbb55b081095622))
- fix intra-doc link references ([73549ff](https://github.com/urmzd/gymnasia/commit/73549ff6525ac346732d6424176a4bf37852c27c))
- **templates**: simplify pull request template ([62489fe](https://github.com/urmzd/gymnasia/commit/62489fec9531d1a8ecb81d2154b35aef0b9a7d66))
- **usage**: update to version 3 and modernize commands ([89b9b43](https://github.com/urmzd/gymnasia/commit/89b9b437048b32ae9ca638f3e949254507e23b64))
- **contributing**: rewrite with workflow and examples ([b200fbb](https://github.com/urmzd/gymnasia/commit/b200fbb78625ad12a852196d623de6a63c969558))
- update architecture and feature documentation ([a45e429](https://github.com/urmzd/gymnasia/commit/a45e4295ffec768c01a2a969db0735f97772ae8a))
- **readme**: add development guide and fix example imports ([5850f4d](https://github.com/urmzd/gymnasia/commit/5850f4d7507fbe3bce5a47a02cd4500e6c3c88a6))
- **core**: improve documentation for Vec<f64> flat_dim ([3646840](https://github.com/urmzd/gymnasia/commit/364684005054a7ad98daff26b0d965d4a7a0e1f8))
- update README, ROADMAP, and AGENTS.md for v3.0.0 ([c4a583d](https://github.com/urmzd/gymnasia/commit/c4a583d6217023ab092f60df36286df56f2b72b9))
- add roadmap and update AGENTS.md for v3 ([e00d5a9](https://github.com/urmzd/gymnasia/commit/e00d5a9f5ea49e3be4c0677c2b3a95188f10136f))

### Refactoring

- **wrappers**: use VecDeque for episode history ([950df79](https://github.com/urmzd/gymnasia/commit/950df79d5333b0b33b88f411104937026bf9d600))
- **wrappers**: lazy-initialize NormalizeObservation statistics ([fe3d9c7](https://github.com/urmzd/gymnasia/commit/fe3d9c7eaeff32364361a0f58e55570e234e8713))
- **wrappers**: flatten observations with per-element bounds ([0d8e2c3](https://github.com/urmzd/gymnasia/commit/0d8e2c345e0d2c2103f6521db8bbe93decade9da))
- **wrappers**: move AsBoxBounds trait to module ([2d582c7](https://github.com/urmzd/gymnasia/commit/2d582c784beb102a43748ab8baf0ab47bd24bcbf))
- **wrappers**: use Bounded::clamp in ClipAction wrapper ([176e11e](https://github.com/urmzd/gymnasia/commit/176e11e0a1a63de0e9b9ebf79116dd96fe242e5d))
- **justfile**: simplify and streamline build targets ([ffa7455](https://github.com/urmzd/gymnasia/commit/ffa74553a4f789de434db51f3e5d18138f690715))
- **render**: simplify RenderEnv import path ([fcb669b](https://github.com/urmzd/gymnasia/commit/fcb669be179020754fc31667ddfeeb5399d1e421))

### Miscellaneous

- format code with cargo fmt ([f13c82d](https://github.com/urmzd/gymnasia/commit/f13c82dbd7976342dbb0747a1a3fa7580afef499))
- **deps**: remove derive-new dependency ([43b0695](https://github.com/urmzd/gymnasia/commit/43b0695700f0dc149e8f62af03fe830c60c715b7))
- **workflows**: use stable rust toolchain instead of nightly ([e1b070e](https://github.com/urmzd/gymnasia/commit/e1b070e4cef04e7109bf697fb43c2e48ae1a683c))
- remove unused project configuration files ([1b4308c](https://github.com/urmzd/gymnasia/commit/1b4308c36bdde5c1e2454cef8bb8e2d874031193))
- **lib**: remove unused_crate_dependencies lint warning ([1eb5e7a](https://github.com/urmzd/gymnasia/commit/1eb5e7ace163f240625af808d4fc9d33cb117068))
- **examples**: sort imports alphabetically ([5ad6bed](https://github.com/urmzd/gymnasia/commit/5ad6bed5b74aefa07c836cc614e5a39b80285273))
- **showcase**: update demonstration GIFs ([e5a8530](https://github.com/urmzd/gymnasia/commit/e5a8530eecb854482149d3969e51510ae6809d93))
- **spaces**: format multi discrete assertion ([e064737](https://github.com/urmzd/gymnasia/commit/e064737a5d4e2a795f934cac1fd53b11d9fbf1b7))
- **envs**: format environment method signatures ([ef27980](https://github.com/urmzd/gymnasia/commit/ef279808a30eb312478ecef4d63b302e9b27a34e))
- **wrappers**: sort module declarations and exports ([f44d8b9](https://github.com/urmzd/gymnasia/commit/f44d8b9e64c7921e5c7e9a77928a992efe8c6f46))
- **wrappers**: format wrapper trait implementations ([4051b4a](https://github.com/urmzd/gymnasia/commit/4051b4a46c8b79e01fef2ddb547c9cc8087618e9))
- clear rustfmt configuration ([1718257](https://github.com/urmzd/gymnasia/commit/17182574fcc64dd535581a5797d64e8c98081291))
- add linguist overrides to fix language stats ([5a7c776](https://github.com/urmzd/gymnasia/commit/5a7c7760b1dfc93c85c43e8ca1c4961d3ffc7ba5))
- update sr action from v2 to v3 ([98fd7d1](https://github.com/urmzd/gymnasia/commit/98fd7d152f98bbd4baf29d68d43ea55e1c92707d))

[Full Changelog](https://github.com/urmzd/gymnasia/compare/v2.0.0...v3.0.0)


## 2.0.0 (2026-03-30)

### Breaking Changes

- decouple simulation from rendering, replace SDL2 with macroquad ([a02c8c5](https://github.com/urmzd/gymnasia/commit/a02c8c570cd0abb2cd591259b91aba1d34509afc))

### Documentation

- update README (#2) ([b62c3fb](https://github.com/urmzd/gymnasia/commit/b62c3fb8476b5bc8e2575ec9d4b26cbb7a80cb70))
- **skills**: align SKILL.md with agentskills.io spec ([7dc9260](https://github.com/urmzd/gymnasia/commit/7dc92602b47b295ae6f0c1af6b009395b585b09d))
- add showcase screenshot ([62716cf](https://github.com/urmzd/gymnasia/commit/62716cf2e6dd063b1bda218870e1ae650300ceef))
- add showcase section to README ([d0ec784](https://github.com/urmzd/gymnasia/commit/d0ec784ef324fac4348aa4e490198e6467bfc2de))

### Miscellaneous

- standardize sr.yaml and justfile — floating_tags true, refactor bump, ci/record recipes ([92271b9](https://github.com/urmzd/gymnasia/commit/92271b9ecd550d8feb86ac1d6e1e3da6618db287))
- use sr-releaser GitHub App for release workflow (#1) ([e151802](https://github.com/urmzd/gymnasia/commit/e151802cbbb016422126754ec77733bc106d13d3))
- update semantic-release action to sr@v2 ([b260413](https://github.com/urmzd/gymnasia/commit/b260413c311a1ce04a6889fe6a28b2cf33b3814a))
- display example images in styled HTML table ([facddf5](https://github.com/urmzd/gymnasia/commit/facddf5f9f9b87144f383bbc3e3e095e9de82811))
- remove Buildkite pipeline config ([f8de89f](https://github.com/urmzd/gymnasia/commit/f8de89fdb0654461abe8790a0d163259797d4216))

[Full Changelog](https://github.com/urmzd/gymnasia/compare/v1.1.0...v2.0.0)


## 1.0.1 (2026-02-27)

### Bug Fixes

- **ci**: remove duplicate sr release call that skipped publish ([586f1fe](https://github.com/urmzd/gymnasia/commit/586f1fef4c37935572f680ee904c2f5996d765ac))

### Miscellaneous

- **release**: v1.0.0 [skip ci] ([9f9e62a](https://github.com/urmzd/gymnasia/commit/9f9e62ac7eb23d314bdf6f8951f52ce5378bffee))


## 1.0.0 (2026-02-27)


## 1.0.0 (2026-02-27)

### Breaking Changes

- rebrand gym-rs to gymnasia v1.0.0 ([9fc4b9e](https://github.com/urmzd/gymnasia/commit/9fc4b9e111632dc7b14dbcfe07566cf93bbb93b1))

### Features

- make SDL2 an optional dependency for headless usage ([223a0a0](https://github.com/urmzd/gymnasia/commit/223a0a0ba95147145a01fd755d43a4659222d78a))

### Bug Fixes

- resolve clippy warnings and fix headless render panic ([063212f](https://github.com/urmzd/gymnasia/commit/063212fc7d24314898085e007dc03c23b8578bda))
- replace manual Default impl with derive attribute for RenderMode ([683c5f3](https://github.com/urmzd/gymnasia/commit/683c5f3523d9fdae0ea7efa601c04e37b5f6515a))
- add macOS framework dependencies for bundled SDL2 build ([148280f](https://github.com/urmzd/gymnasia/commit/148280f61dfd30b991a9385570a3773024eeed21))

### Documentation

- update README for optional SDL2 and bump version to 0.4.0 ([6cfd84d](https://github.com/urmzd/gymnasia/commit/6cfd84dbeacc6afaf8a8a473fac2d4793964fd07))
- extend usage ([3aae498](https://github.com/urmzd/gymnasia/commit/3aae49840c32b5e8c079dc528d5315e207ddaf6f))
- remove unused sections ([6b45d36](https://github.com/urmzd/gymnasia/commit/6b45d36db98ecdaaef38a1e55d8dbfd13029eaac))
- define dependency to sdl2 ([2c55c10](https://github.com/urmzd/gymnasia/commit/2c55c10bcb09fb1288b72b247016520993c4af99))
- update README to include pre-reqs ([ebf2f8e](https://github.com/urmzd/gymnasia/commit/ebf2f8eab315aac04a81b16e198e1205dc9c98ab))
- describe the purpose of utility namespaces ([72318a2](https://github.com/urmzd/gymnasia/commit/72318a2c6f56466f75f3fdefc47ef9489af9b8b2))
- describe the purpose of the operations on the renderer object. ([9d4cf6d](https://github.com/urmzd/gymnasia/commit/9d4cf6d8cfbcb15554b9069deefbd819b012258e))
- describe when an episode ends ([e013cc6](https://github.com/urmzd/gymnasia/commit/e013cc636b50d85ea8c3d0f7752e7c9001ad3fb6))
- document mountain car environment attributes ([d8ca52d](https://github.com/urmzd/gymnasia/commit/d8ca52d876b8631ed57b9a5a936f7fc2145e73f6))
- remove unused mountain documetation ([7b52a66](https://github.com/urmzd/gymnasia/commit/7b52a66e905a2947c7fa6c15b8434d710406656b))
- describe the properties in a cartpole environment. ([6e13969](https://github.com/urmzd/gymnasia/commit/6e139690aac5238e5df597bfef5a2665e2aba67a))
- **cartpole**: reduce the number of fields exposed and document public namespaces ([a321759](https://github.com/urmzd/gymnasia/commit/a3217590cbc8b05ebf5701e3627e740e965c4c50))
- **cartpole**: describe environment and state ([3008723](https://github.com/urmzd/gymnasia/commit/30087230f21d8a5c1c8bd76617b68b734be92878))
- **render**: define the purpose of each render mode ([6288e34](https://github.com/urmzd/gymnasia/commit/6288e347735d26aeca9f1ef950965e7a7715372d))
- **spaces**: describe the purpose of each space ([ccb4447](https://github.com/urmzd/gymnasia/commit/ccb444766f7e174b9c8047c880ae3f092baccf26))
- **utils**: describe the purpose of each utility namespace ([bc864cb](https://github.com/urmzd/gymnasia/commit/bc864cb9b957ac4e0a9824caec46db3681485595))
- **utils**: describe the purpose of the renderer object ([5d0dc2e](https://github.com/urmzd/gymnasia/commit/5d0dc2e0e0ccd0c4449efc23e75db5822479c977))

### Refactoring

- remove deadcode. ([7393dd5](https://github.com/urmzd/gymnasia/commit/7393dd56fd968d8f586d5d292d99d1bd461f0c76))
- custom code split across multiple files ([6673938](https://github.com/urmzd/gymnasia/commit/66739384d364a4ff4b7c5343c8c2fd11263a9925))
- move example tests to examples ([709836a](https://github.com/urmzd/gymnasia/commit/709836a5c8b95823a1cbeba47e15f317feb7f9d7))

### Miscellaneous

- apply rustfmt import ordering ([7866396](https://github.com/urmzd/gymnasia/commit/7866396185ce2e40c3e20e3b098fba51d7d9060f))
- clean up .gitignore and remove tracked log file ([1b23b37](https://github.com/urmzd/gymnasia/commit/1b23b37da19b8d638e343fb56e9e9211c6880e61))
- clean up .gitignore and remove tracked log file ([2e7b133](https://github.com/urmzd/gymnasia/commit/2e7b1335f9546467edeca31aec27733d54927e92))
- add CI/CD pipelines and semantic release configuration ([347f70a](https://github.com/urmzd/gymnasia/commit/347f70a65a34f9344604d0d4d05fa1b386db020b))
- stop tracking .direnv/flake-profile ([e6c355e](https://github.com/urmzd/gymnasia/commit/e6c355e24e0df6cc1dd25a26638a273ed60707d6))
- add sensitive paths to .gitignore ([a843bd0](https://github.com/urmzd/gymnasia/commit/a843bd06efc96cf2fe4a486d4b56797de5c11f19))
- resolve clippy lints ([5283afa](https://github.com/urmzd/gymnasia/commit/5283afaa86a3a7c45c46c882cfad459f02539b62))
- bump all outdated dependencies ([f36e6aa](https://github.com/urmzd/gymnasia/commit/f36e6aafd61c2ce8ca7e80c9a1df609e9d12ba39))
- clip util ([7e5a923](https://github.com/urmzd/gymnasia/commit/7e5a92380e001dbcae1843e6098eb8909fce6cf8))
- discrete space ([41fd746](https://github.com/urmzd/gymnasia/commit/41fd746e3009af5bcd249cd30ae323006d4cc3c8))
- add cartpole example ([bc593fe](https://github.com/urmzd/gymnasia/commit/bc593fe2ea7989a40e19cc5e95d9eadc1261a9ec))
