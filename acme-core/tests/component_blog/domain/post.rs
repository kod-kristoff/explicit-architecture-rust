/*
 * This file is part of the Explicit Architecture POC,
 * which is created on top of the Symfony Demo application.
 *
 * (c) Herberto Graça <herberto.graca@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

use acme_core::component::blog::domain::post::Post;

use rstest::rstest;
/**
 * @small
 *
 * @internal
 */

/**
 * @test
 * @dataProvider provideSuffixes
 */
#[rstest]
#[case("15")]
#[case("-15")]
fn postfix_slug(#[case] suffix: &str) {
    let title = "Some Interesting Title";

    let mut post = Post::new();
    post.setTitle(title);
    let slug = post.getSlug().to_string();

    post.postfixSlug(suffix).unwrap();

    assert_eq!(
        post.getSlug(),
        &format!("{}-{}", slug, suffix.replace('-', ""))
    );
}
