// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Pull in every day that was created.
automod::dir!(pub "src/days");

// TODO: Find a better way to do this.
pub const DAYS: [fn(&str); 1] = [day01::day01];
