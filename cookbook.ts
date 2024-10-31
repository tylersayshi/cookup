type Recipe = {
  created_at: string;
  name: string;
  ingredients: string;
  instructions: string;
};

export default [
  {
    created_at: "2024-10-01",
    name: "Pesto and sundried tomato stuffed zucchini boats",
    ingredients: `- 3 medium zucchinis
- 1/2 cup of pesto
- 1/4 cup of sundried tomatoes, chopped
- 1/4 cup of grated parmesan cheese
- Salt and pepper to taste
`,
    instructions: `1. Preheat your oven to 375°F (190°C).
2. Cut the zucchinis in half lengthwise and scoop out the seeds and flesh from the center to create a hollow "boat".
3. In a mixing bowl, combine the pesto, chopped sundried tomatoes, and grated parmesan cheese. Season with salt and pepper to taste.
4. Spoon the pesto mixture into the hollowed-out zucchini boats, filling them generously.
5. Place the filled zucchini boats on a baking sheet lined with parchment paper.
6. Bake in the preheated oven for about 20-25 minutes, or until the zucchinis are tender and the filling is heated through.
7. Serve the pesto and sundried tomato stuffed zucchini boats hot as a delicious and satisfying meal or side dish. Enjoy!
`,
  },
  {
    created_at: "2024-10-01",
    name: "Chocolate Chip Cookies",
    ingredients: `- 310 g flour
- 1/2 tsp baking soda
- 1/2 tsp salt
- 170 g butter (melted)
- 1 cup brown sugar
- 1/2 cup sugar
- 1 tbsp vanilla
- 1 egg & 1 egg yolk
- 2 cups chocolate chips
`,
    instructions: `1. preheat 325 & grease cookie sheets
2. sift flour, baking soda, & salt
3. mix butter & sugar till well blended. beat in vanilla & eggs. mix in dry ingredients. add chocolate chips with spoon, then make big cookies on the sheets.
4. bake 15-17 mins
`,
  },
] satisfies Recipe[];
