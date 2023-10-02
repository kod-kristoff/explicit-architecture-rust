/*
 * This file is part of the Explicit Architecture POC,
 * which is created on top of the Symfony Demo application.
 *
 * (c) Herberto Graça <herberto.graca@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

use std::fmt;

#[derive(Debug)]
pub struct SlugIsImmutableError;
impl fmt::Display for SlugIsImmutableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("A post slug can only be changed during the post creation,")?;
        f.write_str(" after that it is immutable so the post retains the SEO value.")
    }
}

impl std::error::Error for SlugIsImmutableError {}
