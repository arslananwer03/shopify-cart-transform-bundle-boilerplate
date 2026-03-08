let bundleId = `bundle-${Date.now()}`; // Unique ID for this specific bundle

let formData = {
  'items': [
    { 'id': 44123456789, 'quantity': 1, 'properties': { '_bundle_id': bundleId } },
    { 'id': 44987654321, 'quantity': 1, 'properties': { '_bundle_id': bundleId } }
  ]
};

fetch('/cart/add.js', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify(formData)
});
