# Changelog

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
