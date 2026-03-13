.PHONY: api web dev check clean reset-db

# Run the API server on :3000
api:
	cargo run -p api

# Run the frontend dev server on :8080
web:
	cd crates/web && trunk serve

# Run both API and frontend in parallel
dev:
	$(MAKE) api & $(MAKE) web & wait

# Type-check the entire workspace
check:
	cargo check

# Delete build artifacts and database
clean:
	cargo clean
	rm -rf crates/web/dist crates/web/tailwind-output.css data.db

# Reset the database (will be re-seeded on next API start)
reset-db:
	rm -f data.db
