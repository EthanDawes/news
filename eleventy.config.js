const config = {
  pathPrefix: "/news",
  dir: {
    input: "src",
  }
};

export default function(eleventyConfig) {
  eleventyConfig.addPassthroughCopy("src/**/*.{png,jpg}");

  eleventyConfig.addTemplateFormats("mjml");

  eleventyConfig.addExtension("mjml", {
		compile: async (inputContent) => {
			// Replace any instances of cloud with butt
			let output = inputContent.replace(/cloud/gi, "butt");

			return async () => {
				return output;
			};
		},
	});

  eleventyConfig.addCollection("posts", function (collectionApi) {
		return collectionApi.getFilteredByGlob("src/[0-9][0-9][0-9][0-9]/**");
	});

  eleventyConfig.addGlobalData("eleventyComputed", {
    permalink: (data) => {
      if (data.page.inputPath.match(/[0-9]{4}/)) {
        // Permalink format for posts: /<year>/<short month>/
        const year = new Intl.DateTimeFormat('en', { year: 'numeric' }).format(data.page.date);
        const month = new Intl.DateTimeFormat('en', { month: 'short' }).format(data.page.date);
        return `/${year}/${month}/`;
      }
      return data.permalink; // Preserve any existing permalink or default
    },
  });

  return config;
};