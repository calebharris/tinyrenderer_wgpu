---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: default
---

{% for post in site.posts %}
## {{ post.title }}
### {{ post.date | date_to_string: "ordinal", "US" }}
{{ post.excerpt }}
[Continue reading...]({{post.url | relative_url}})
{% endfor %}
