<!--
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0.  If a copy of the MPL was not distributed with this
  file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Joyscript

**Joyscript** is a strongly typed, concatinative, homoiconic programming language intended for use in the construction of domain-specific languages in high-assurance software systems.
The design of Joyscript is inspired in roughly equal parts by **Forth**, **Joy**, **Haskell**, and **Lisp** and it is very closely related to **Mirth**.

## Directory structure

- `bootstrap`: The bootstrap compiler and runtime environment, in the form of a minimal Joyscript -> Rust source-level converter.
- `src`: The self-hosted Joyscript compiler and runtime environment, written in the subset of Joyscript understood by the bootstrap compiler.

  - `tools/joyscript-code`: Joyscript extension for Visual Studio Code.
  - `tools/joyscript-mode`: Joyscript syntax highlighting for Emacs.
  - `tools/joyscript-vim`: Joyscript syntax highlighting for Vim.

## Contributing to Joyscript

Please use the project's GitHub [issue tracker](https://github.com/maaku/joyscript/issues) for bug reports and requests.

To contribute directly, open a pull request against the `master` branch of the project's GitHub [repository](https://github.com/maaku/joyscript).
All work must be contributed under the MPL2.0 license (see below).
Add the following notice at the top of any new files:

> This Source Code Form is subject to the terms of the Mozilla Public
> License, v. 2.0.  If a copy of the MPL was not distributed with this
> file, You can obtain one at https://mozilla.org/MPL/2.0/.

If you see anything that can be improved, please consider submitting a pull request!

## License

The entire code base is licensed under the Mozilla Public License 2.0.

This license gives anyone the right to view, share, modify, and contribute source code, as long as the license notice is preserved at the top of every source code file.
The code base (or portions and/or modifications thereof) can be used or distributed as part of a larger project, as long as the license notice is preserved, or in binary form, as long as source code is made available to the user.
For the avoidance of doubt, this paragraph is merely informative and only the license text matters.

Please read the license text under `LICENSE.txt` or [here](https://mozilla.org/MPL/2.0/).
