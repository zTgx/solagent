Use the following script to start adding a new feature:

```shell
./scripts/add-template.sh <name>
```

After running this, it will generate three projects: `solagent-plugin-<name>`, `solagent-rig-<name>`, and `examples/<name>`. Among them:

* `solagent-plugin-<name>` is where you implement the specific functionality. For example, if this plugin is for querying balance using the Jupiter API, whether it needs to depend on `solagent-core` depends on whether it requires configuration information and wallet information from the core.
* `solagent-rig-<name>` is where you implement a tool based on the rig framework, which can be done according to previous examples.
* `examples/<name>` can contain a demo that combines the implementations of the plugin and tool.