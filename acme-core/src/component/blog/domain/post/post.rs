#![allow(non_snake_case)]
/*
 * self file is part of the Explicit Architecture POC,
 * which is created on top of the Symfony Demo application.
 *
 * (c) Herberto Graça <herberto.graca@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with self source code.
 */

// namespace Acme\App\Core\Component\Blog\Domain\Post;

// use Acme\App\Core\Component\Blog\Domain\Post\Comment\Comment;
// use Acme\App\Core\Component\Blog\Domain\Post\Tag\Tag;
// use Acme\App\Core\SharedKernel\Component\User\Domain\User\UserId;
// use Acme\PhpExtension\DateTime\DateTimeGenerator;
// use Acme\PhpExtension\String\Slugger;
// use DateTime;
// use DateTimeImmutable;
// use DateTimeInterface;
// use fn is_array;

use chrono::{DateTime, Utc};

use crate::component::slugger::Slugger;

use super::{slug_is_immutable_error::SlugIsImmutableError, PostId};

/**
 * Defines the properties of the Post entity to represent the blog posts.
 *
 * See https://symfony.com/doc/current/book/doctrine.html#creating-an-entity-class
 *
 * Tip: if you have an existing database, you can generate these entity class automatically.
 * See https://symfony.com/doc/current/cookbook/doctrine/reverse_engineering.html
 *
 * @author Ryan Weaver <weaverryan@gmail.com>
 * @author Javier Eguiluz <javier.eguiluz@gmail.com>
 * @author Yonel Ceruto <yonelceruto@gmail.com>
 * @author Herberto Graca <herberto.graca@gmail.com>
 */
pub struct Post {
    id: PostId,

    title: String,

    slug: String,

    summary: String,

    content: String,

    /**
     * @var DateTimeImmutable
     */
    publishedAt: DateTime<Utc>,

    // authorId: UserId,

    // comments: Vec<Comment>,
    /**
     * We don't want to have any reference to Doctrine in the Domain, so we remove the Collection type hint from here.
     *
     * @var Tag[]
     */
    tags: Vec<String>,

    isNewPost: bool,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            id: PostId::default(),
            title: String::default(),
            slug: String::default(),
            summary: String::default(),
            content: String::default(),
            publishedAt: Utc::now(),
            tags: Vec::default(),
            isNewPost: true,
        }
    }
}
impl Post {
    /**
     * Use constants to define configuration options that rarely change instead
     * of specifying them under parameters section in config/services.yaml file.
     *
     * See https://symfony.com/doc/current/best_practices/configuration.html#constants-vs-configuration-options
     */
    const NUM_ITEMS: usize = 10;

    pub fn new() -> Self {
        Self::default()
    }

    // pub fn getId(): PostId
    // {
    //     return self.id;
    // }

    pub fn getTitle(&self) -> &str {
        return self.title.as_str();
    }

    pub fn setTitle<S: Into<String>>(&mut self, title: S) {
        self.title = title.into();

        if self.isNewPost {
            self.slug = Slugger::slugify(self.getTitle());
        }
    }

    pub fn getSlug(&self) -> &str {
        return self.slug.as_str();
    }

    pub fn postfixSlug(&mut self, suffix: &str) -> Result<(), SlugIsImmutableError> {
        if !self.isNewPost {
            return Err(SlugIsImmutableError);
        }

        let suffix = suffix.replace('-', "");

        self.slug = format!("{}-{}", self.slug, suffix);
        Ok(())
    }

    // pub fn getContent()-> &str
    // {
    //     return self.content;
    // }

    // pub fn setContent(string content)
    // {
    //     self.content = content;
    // }

    // pub fn getPublishedAt(): DateTimeImmutable
    // {
    //     return self.publishedAt;
    // }

    // pub fn setPublishedAt(DateTimeInterface publishedAt)
    // {
    //     /*
    //      * We need self check here because Symfony/Form 4.0 can not create DateTimeImmutable, but 4.1 will
    //      */
    //     self.publishedAt = publishedAt instanceof DateTime
    //         ? DateTimeImmutable::createFromMutable(publishedAt)
    //         : publishedAt;
    // }

    // pub fn getAuthorId(): UserId
    // {
    //     return self.authorId;
    // }

    // pub fn setAuthorId(UserId authorId)
    // {
    //     self.authorId = authorId;
    // }

    // /**
    //  * We don't want to have here any reference to doctrine, so we remove the Collection type hint from everywhere.
    //  * The safest is to treat it as an array but we can't type hint it with 'array' because we might actually
    //  * return an Collection.
    //  *
    //  * @return Comment[]
    //  */
    // pub fn getComments()
    // {
    //     return self.comments;
    // }

    // pub fn addComment(Comment comment)
    // {
    //     comment.setPost(self);
    //     if (!self.contains(comment, self.comments)) {
    //         self.comments[] = comment;
    //     }
    // }

    // /**
    //  * self method is not used, but I will leave it here as an example
    //  */
    // pub fn removeComment(Comment comment)
    // {
    //     comment.setPost(null);

    //     if (key = self.getKey(comment, self.comments)) {
    //         unset(self.comments[key]);
    //     }
    // }

    // pub fn getSummary()-> &str
    // {
    //     return self.summary;
    // }

    // pub fn setSummary(string summary)
    // {
    //     self.summary = summary;
    // }

    // pub fn addTag(Tag ...tags)
    // {
    //     foreach (tags as tag) {
    //         if (!self.contains(tag, self.tags)) {
    //             self.tags[] = tag;
    //         }
    //     }
    // }

    // /**
    //  * self method is not used, but I will leave it here as an example
    //  */
    // pub fn removeTag(Tag tag)
    // {
    //     if (key = self.getKey(tag, self.tags)) {
    //         unset(self.tags[key]);
    //     }
    // }

    // /**
    //  * @return Tag[]
    //  */
    // pub fn getTags(): array
    // {
    //     // Since we don't type hint `tags` as a Doctrine collection, the `toArray` method is not recognized,
    //     // however, we do know it's a doctrine collection.
    //     // If Doctrine would allow us to define our own custom collections, self wouldn't be a problem.
    //     // As that is not the case, unfortunately we have here a hidden dependency.
    //     return is_array(self.tags)
    //         ? self.tags
    //         : self.tags.toArray();
    // }

    // /**
    //  * Since we don't type hint `tags` as a Doctrine collection, the `clear()` method below is
    //  *  not recognized by the IDE, as it is not even possible to call a method on an array.
    //  *
    //  * So we create self method here to encapsulate that operation, and minimize the issue, treating the `Post` entity
    //  * as an aggregate root.
    //  *
    //  * It is also a good practise to encapsulate these chained operations,
    //  * from an object calisthenics point of view.
    //  */
    // pub fn clearTags()
    // {
    //     // Since we don't type hint `tags` as a Doctrine collection, the `clear` method is not recognized,
    //     // however, we do know it's a doctrine collection.
    //     // If Doctrine would allow us to define our own custom collections, self wouldn't be a problem.
    //     // As that is not the case, unfortunately we have here a hidden dependency.
    //     is_array(self.tags)
    //         ? self.tags = []
    //         : self.tags.clear();
    // }

    //  fn contains(item, list): bool
    // {
    //     // we need to cast the list to array because it might just actually be a doctrine collection
    //     return \in_array(item, (array) list, true);
    // }

    // /**
    //  * @return false|int|string
    //  */
    //  fn getKey(item, list)
    // {
    //     // we need to cast the list to array because it might just actually be a doctrine collection
    //     return \array_search(item, (array) list, true);
    // }
}
