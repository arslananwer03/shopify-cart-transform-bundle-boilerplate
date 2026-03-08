📦 The "Boom" Bundle Blueprint
Building Multi-Price Dynamic Bundles with Shopify Cart Transform API

The Story: From "Messy Cart" to "Clean Checkout"
In the old days of Shopify, if a customer wanted to "Build Their Own Box," they ended up with 5 separate items in their cart. It looked cluttered, discounts were hard to manage, and the customer experience felt "broken."

The Boom Method changes the narrative. By using the Cart Transform API, we let the customer pick whatever they want, and—Boom—the moment it hits the cart, it transforms into a single, beautifully priced bundle.

Phase 1: The App Container (The "Garage")
Unlike Liquid, this logic lives in a Custom App. Think of this as the "engine room" for your store.

Initialize: Open your terminal and run shopify app init.

Scaffold the Function: Run shopify app generate extension. Select Cart Transform.

Choose Your Language: Rust is the gold standard for speed in 2026, but JavaScript is also supported.

Requirement: You must be on Shopify Plus or a Development Store to use the merge and lineUpdate operations.

Phase 2: The Frontend (The "Add-to-Cart" Spark)
Your theme needs to tag items so the backend knows they belong together. We use a unique _bundle_id for every single box a user builds.

The Workflow:

The user picks 3 products.

Your JS generates a unique ID: const myId = 'bundle_' + Date.now();.

You push them to the cart using the AJAX API with that ID as a property.

Phase 3: The Backend (The "Transformation" Logic)
This is where your Rust code sits and waits. When it sees items with matching _bundle_id properties, it triggers a merge operation.

How we handle Multiple Bundles:
The code doesn't just look for one bundle. It groups the entire cart by the _bundle_id attribute. If a user builds two different boxes, they get two different IDs, and the function creates two separate merge operations.

The Pricing Logic:
In your run.rs (or run.js), you can define the price for each specific bundle:

Bundle A: Fixed price of $50.00.

Bundle B: 15% off the total of the items inside.

Phase 4: Deployment & Activation
Deploy: shopify app deploy.

Install: Install the custom app on your store via the Partner Dashboard.

Activate: Crucial step! Go to Shopify Admin > Settings > Apps and sales channels. Click on your app and ensure the Cart Transform extension is toggled ON.

Expert Warnings & Workarounds
The "One Function" Rule: You can only have one Cart Transform function active per store. If you have a conflict, you must combine your logic into a single app.

The "Same Parent" Bug: If multiple bundles use the same "Parent Product," they might merge into one in the cart. Fix: Give each bundle a unique title in your function output (e.g., "Gift Box #1", "Gift Box #2").

The Subscription Wall: Shopify currently rejects merge operations if any item in the bundle has a "Selling Plan" (Subscription) attached.
