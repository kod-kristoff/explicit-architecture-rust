/*
 * This file is part of the Explicit Architecture POC,
 * which is created on top of the Symfony Demo application.
 *
 * (c) Herberto Graça <herberto.graca@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

use ulid::Ulid;

pub struct PostId(Ulid);

impl Default for PostId {
    fn default() -> Self {
        Self::gen()
    }
}
impl PostId {
    pub fn new(value: Ulid) -> Self {
        Self(value)
    }

    pub fn gen() -> Self {
        Self::new(Ulid::new())
    }
}
