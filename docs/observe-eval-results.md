# Observe Precision Evaluation Results

## Summary

| Metric | Value |
|--------|-------|
| TP (correct predictions) | 155 |
| FP (incorrect predictions) | 6 |
| FN (missed ground truth) | 11 |
| Ignored (secondary targets) | 178 |
| Precision | 96.3% |
| Recall | 93.4% |
| F1 Score | 94.8% |

## Stratum Breakdown

| Evidence Type | GT Pairs | TP | FN | Recall |
|---------------|----------|----|----|--------|
| barrel_import | 32 | 25 | 7 | 78.1% |
| call_usage | 69 | 63 | 6 | 91.3% |
| constructor_usage | 97 | 95 | 2 | 97.9% |
| direct_import | 134 | 130 | 4 | 97.0% |
| filename_match | 124 | 123 | 1 | 99.2% |
| provider_registration | 3 | 3 | 0 | 100.0% |
| symbol_assertion | 46 | 36 | 10 | 78.3% |
| test_name_match | 154 | 144 | 10 | 93.5% |

## True Positives

| Test File | Production File |
|-----------|-----------------|
| packages/common/test/decorators/apply-decorators.spec.ts | packages/common/decorators/core/apply-decorators.ts |
| packages/common/test/decorators/bind.decorator.spec.ts | packages/common/decorators/core/bind.decorator.ts |
| packages/common/test/decorators/catch.decorator.spec.ts | packages/common/decorators/core/catch.decorator.ts |
| packages/common/test/decorators/controller.decorator.spec.ts | packages/common/decorators/core/controller.decorator.ts |
| packages/common/test/decorators/create-param-decorator.spec.ts | packages/common/decorators/http/create-route-param-metadata.decorator.ts |
| packages/common/test/decorators/dependencies.decorator.spec.ts | packages/common/decorators/core/dependencies.decorator.ts |
| packages/common/test/decorators/exception-filters.decorator.spec.ts | packages/common/decorators/core/exception-filters.decorator.ts |
| packages/common/test/decorators/global.decorator.spec.ts | packages/common/decorators/modules/global.decorator.ts |
| packages/common/test/decorators/header.decorator.spec.ts | packages/common/decorators/http/header.decorator.ts |
| packages/common/test/decorators/http-code.decorator.spec.ts | packages/common/decorators/http/http-code.decorator.ts |
| packages/common/test/decorators/inject.decorator.spec.ts | packages/common/decorators/core/inject.decorator.ts |
| packages/common/test/decorators/injectable.decorator.spec.ts | packages/common/decorators/core/injectable.decorator.ts |
| packages/common/test/decorators/module.decorator.spec.ts | packages/common/decorators/modules/module.decorator.ts |
| packages/common/test/decorators/redirect.decorator.spec.ts | packages/common/decorators/http/redirect.decorator.ts |
| packages/common/test/decorators/render.decorator.spec.ts | packages/common/decorators/http/render.decorator.ts |
| packages/common/test/decorators/request-mapping.decorator.spec.ts | packages/common/decorators/http/request-mapping.decorator.ts |
| packages/common/test/decorators/route-params.decorator.spec.ts | packages/common/decorators/http/route-params.decorator.ts |
| packages/common/test/decorators/set-metadata.decorator.spec.ts | packages/common/decorators/core/set-metadata.decorator.ts |
| packages/common/test/decorators/sse.decorator.spec.ts | packages/common/decorators/http/sse.decorator.ts |
| packages/common/test/decorators/use-guards.decorator.spec.ts | packages/common/decorators/core/use-guards.decorator.ts |
| packages/common/test/decorators/use-interceptors.decorator.spec.ts | packages/common/decorators/core/use-interceptors.decorator.ts |
| packages/common/test/decorators/use-pipes.decorator.spec.ts | packages/common/decorators/core/use-pipes.decorator.ts |
| packages/common/test/decorators/version.decorator.spec.ts | packages/common/decorators/core/version.decorator.ts |
| packages/common/test/file-stream/streamable-file.spec.ts | packages/common/file-stream/streamable-file.ts |
| packages/common/test/module-utils/configurable-module.builder.spec.ts | packages/common/module-utils/configurable-module.builder.ts |
| packages/common/test/module-utils/utils/get-injection-providers.util.spec.ts | packages/common/module-utils/utils/get-injection-providers.util.ts |
| packages/common/test/pipes/default-value.pipe.spec.ts | packages/common/pipes/default-value.pipe.ts |
| packages/common/test/pipes/file/file-type.validator.spec.ts | packages/common/pipes/file/file-type.validator.ts |
| packages/common/test/pipes/file/max-file-size.validator.spec.ts | packages/common/pipes/file/max-file-size.validator.ts |
| packages/common/test/pipes/file/parse-file-pipe.builder.spec.ts | packages/common/pipes/file/file-type.validator.ts |
| packages/common/test/pipes/file/parse-file-pipe.builder.spec.ts | packages/common/pipes/file/parse-file-pipe.builder.ts |
| packages/common/test/pipes/file/parse-file.pipe.spec.ts | packages/common/pipes/file/parse-file.pipe.ts |
| packages/common/test/pipes/parse-array.pipe.spec.ts | packages/common/pipes/parse-array.pipe.ts |
| packages/common/test/pipes/parse-bool.pipe.spec.ts | packages/common/pipes/parse-bool.pipe.ts |
| packages/common/test/pipes/parse-date.pipe.spec.ts | packages/common/pipes/parse-date.pipe.ts |
| packages/common/test/pipes/parse-enum.pipe.spec.ts | packages/common/pipes/parse-enum.pipe.ts |
| packages/common/test/pipes/parse-float.pipe.spec.ts | packages/common/pipes/parse-float.pipe.ts |
| packages/common/test/pipes/parse-int.pipe.spec.ts | packages/common/pipes/parse-int.pipe.ts |
| packages/common/test/pipes/parse-uuid.pipe.spec.ts | packages/common/pipes/parse-uuid.pipe.ts |
| packages/common/test/pipes/validation.pipe.spec.ts | packages/common/pipes/validation.pipe.ts |
| packages/common/test/serializer/class-serializer.interceptor.spec.ts | packages/common/serializer/class-serializer.interceptor.ts |
| packages/common/test/services/logger.service.spec.ts | packages/common/services/console-logger.service.ts |
| packages/common/test/services/logger.service.spec.ts | packages/common/services/logger.service.ts |
| packages/common/test/services/utils/filter-log-levels.util.spec.ts | packages/common/services/utils/filter-log-levels.util.ts |
| packages/common/test/services/utils/is-log-level-enabled.util.spec.ts | packages/common/services/logger.service.ts |
| packages/common/test/services/utils/is-log-level-enabled.util.spec.ts | packages/common/services/utils/is-log-level-enabled.util.ts |
| packages/common/test/utils/forward-ref.util.spec.ts | packages/common/utils/forward-ref.util.ts |
| packages/common/test/utils/load-package.util.spec.ts | packages/common/utils/load-package.util.ts |
| packages/common/test/utils/merge-with-values.util.spec.ts | packages/common/utils/merge-with-values.util.ts |
| packages/common/test/utils/random-string-generator.util.spec.ts | packages/common/utils/random-string-generator.util.ts |
| packages/common/test/utils/select-exception-filter-metadata.util.spec.ts | packages/common/utils/select-exception-filter-metadata.util.ts |
| packages/common/test/utils/shared.utils.spec.ts | packages/common/utils/shared.utils.ts |
| packages/common/test/utils/validate-each.util.spec.ts | packages/common/utils/validate-each.util.ts |
| packages/core/test/application-config.spec.ts | packages/core/application-config.ts |
| packages/core/test/discovery/discoverable-meta-host-collection.spec.ts | packages/core/discovery/discoverable-meta-host-collection.ts |
| packages/core/test/discovery/discoverable-meta-host-collection.spec.ts | packages/core/injector/instance-wrapper.ts |
| packages/core/test/discovery/discovery-service.spec.ts | packages/core/discovery/discovery-service.ts |
| packages/core/test/discovery/discovery-service.spec.ts | packages/core/injector/module.ts |
| packages/core/test/errors/test/exception-handler.spec.ts | packages/core/errors/exception-handler.ts |
| packages/core/test/errors/test/exceptions-zone.spec.ts | packages/core/errors/exceptions-zone.ts |
| packages/core/test/errors/test/messages.spec.ts | packages/core/errors/messages.ts |
| packages/core/test/exceptions/base-exception-filter.spec.ts | packages/core/exceptions/base-exception-filter-context.ts |
| packages/core/test/exceptions/exceptions-handler.spec.ts | packages/core/exceptions/exceptions-handler.ts |
| packages/core/test/exceptions/external-exception-filter-context.spec.ts | packages/core/exceptions/external-exception-filter-context.ts |
| packages/core/test/exceptions/external-exceptions-handler.spec.ts | packages/core/exceptions/external-exceptions-handler.ts |
| packages/core/test/guards/guards-consumer.spec.ts | packages/core/guards/guards-consumer.ts |
| packages/core/test/guards/guards-context-creator.spec.ts | packages/core/guards/guards-context-creator.ts |
| packages/core/test/helpers/application-ref-host.spec.ts | packages/core/helpers/http-adapter-host.ts |
| packages/core/test/helpers/barrier.spec.ts | packages/core/helpers/barrier.ts |
| packages/core/test/helpers/context-id-factory.spec.ts | packages/core/helpers/context-id-factory.ts |
| packages/core/test/helpers/context-utils.spec.ts | packages/core/helpers/context-utils.ts |
| packages/core/test/helpers/execution-context-host.spec.ts | packages/core/helpers/execution-context-host.ts |
| packages/core/test/helpers/external-context-creator.spec.ts | packages/core/helpers/external-context-creator.ts |
| packages/core/test/helpers/external-context-creator.spec.ts | packages/core/injector/module.ts |
| packages/core/test/helpers/external-proxy.spec.ts | packages/core/helpers/external-proxy.ts |
| packages/core/test/helpers/router-method-factory.spec.ts | packages/core/helpers/router-method-factory.ts |
| packages/core/test/hooks/before-app-shutdown.hook.spec.ts | packages/core/hooks/before-app-shutdown.hook.ts |
| packages/core/test/hooks/before-app-shutdown.hook.spec.ts | packages/core/injector/module.ts |
| packages/core/test/hooks/on-app-bootstrap.hook.spec.ts | packages/core/hooks/on-app-bootstrap.hook.ts |
| packages/core/test/hooks/on-app-bootstrap.hook.spec.ts | packages/core/injector/module.ts |
| packages/core/test/hooks/on-app-shutdown.hook.spec.ts | packages/core/hooks/on-app-shutdown.hook.ts |
| packages/core/test/hooks/on-app-shutdown.hook.spec.ts | packages/core/injector/module.ts |
| packages/core/test/hooks/on-module-destroy.hook.spec.ts | packages/core/hooks/on-module-destroy.hook.ts |
| packages/core/test/hooks/on-module-destroy.hook.spec.ts | packages/core/injector/module.ts |
| packages/core/test/hooks/on-module-init.hook.spec.ts | packages/core/hooks/on-module-init.hook.ts |
| packages/core/test/hooks/on-module-init.hook.spec.ts | packages/core/injector/module.ts |
| packages/core/test/injector/compiler.spec.ts | packages/core/injector/compiler.ts |
| packages/core/test/injector/container.spec.ts | packages/core/injector/container.ts |
| packages/core/test/injector/helpers/provider-classifier.spec.ts | packages/core/injector/helpers/provider-classifier.ts |
| packages/core/test/injector/helpers/silent-logger.spec.ts | packages/core/injector/helpers/silent-logger.ts |
| packages/core/test/injector/injector.spec.ts | packages/common/decorators/core/inject.decorator.ts |
| packages/core/test/injector/injector.spec.ts | packages/common/decorators/core/injectable.decorator.ts |
| packages/core/test/injector/injector.spec.ts | packages/core/injector/injector.ts |
| packages/core/test/injector/injector.spec.ts | packages/core/injector/instance-wrapper.ts |
| packages/core/test/injector/injector.spec.ts | packages/core/injector/module.ts |
| packages/core/test/injector/instance-loader.spec.ts | packages/common/decorators/core/controller.decorator.ts |
| packages/core/test/injector/instance-loader.spec.ts | packages/common/decorators/core/injectable.decorator.ts |
| packages/core/test/injector/instance-loader.spec.ts | packages/core/injector/instance-loader.ts |
| packages/core/test/injector/instance-loader.spec.ts | packages/core/injector/instance-wrapper.ts |
| packages/core/test/injector/instance-wrapper.spec.ts | packages/core/injector/instance-wrapper.ts |
| packages/core/test/injector/internal-core-module/internal-core-module-factory.spec.ts | packages/core/injector/internal-core-module/internal-core-module-factory.ts |
| packages/core/test/injector/lazy-module-loader/lazy-module-loader.spec.ts | packages/core/injector/lazy-module-loader/lazy-module-loader.ts |
| packages/core/test/injector/module.spec.ts | packages/core/injector/instance-wrapper.ts |
| packages/core/test/injector/module.spec.ts | packages/core/injector/module.ts |
| packages/core/test/injector/opaque-key-factory/by-reference-module-opaque-key-factory.spec.ts | packages/core/injector/opaque-key-factory/by-reference-module-opaque-key-factory.ts |
| packages/core/test/injector/opaque-key-factory/deep-hashed-module-opaque-key-factory.spec.ts | packages/core/injector/opaque-key-factory/deep-hashed-module-opaque-key-factory.ts |
| packages/core/test/injector/topology-tree/tree-node.spec.ts | packages/core/injector/topology-tree/tree-node.ts |
| packages/core/test/inspector/graph-inspector.spec.ts | packages/core/injector/instance-wrapper.ts |
| packages/core/test/inspector/graph-inspector.spec.ts | packages/core/injector/module.ts |
| packages/core/test/inspector/graph-inspector.spec.ts | packages/core/inspector/graph-inspector.ts |
| packages/core/test/inspector/serialized-graph.spec.ts | packages/core/inspector/serialized-graph.ts |
| packages/core/test/interceptors/interceptors-consumer.spec.ts | packages/core/interceptors/interceptors-consumer.ts |
| packages/core/test/interceptors/interceptors-context-creator.spec.ts | packages/core/interceptors/interceptors-context-creator.ts |
| packages/core/test/metadata-scanner.spec.ts | packages/core/metadata-scanner.ts |
| packages/core/test/middleware/builder.spec.ts | packages/core/middleware/builder.ts |
| packages/core/test/middleware/container.spec.ts | packages/core/middleware/container.ts |
| packages/core/test/middleware/middleware-module.spec.ts | packages/core/injector/module.ts |
| packages/core/test/middleware/middleware-module.spec.ts | packages/core/middleware/builder.ts |
| packages/core/test/middleware/middleware-module.spec.ts | packages/core/middleware/container.ts |
| packages/core/test/middleware/middleware-module.spec.ts | packages/core/middleware/middleware-module.ts |
| packages/core/test/middleware/resolver.spec.ts | packages/core/middleware/resolver.ts |
| packages/core/test/middleware/route-info-path-extractor.spec.ts | packages/core/middleware/route-info-path-extractor.ts |
| packages/core/test/middleware/routes-mapper.spec.ts | packages/core/middleware/routes-mapper.ts |
| packages/core/test/middleware/utils.spec.ts | packages/core/middleware/utils.ts |
| packages/core/test/nest-application-context.spec.ts | packages/core/nest-application-context.ts |
| packages/core/test/nest-application.spec.ts | packages/core/nest-application.ts |
| packages/core/test/pipes/params-token-factory.spec.ts | packages/core/pipes/params-token-factory.ts |
| packages/core/test/pipes/pipes-consumer.spec.ts | packages/core/pipes/pipes-consumer.ts |
| packages/core/test/pipes/pipes-context-creator.spec.ts | packages/core/pipes/pipes-context-creator.ts |
| packages/core/test/repl/assign-to-object.util.spec.ts | packages/core/repl/assign-to-object.util.ts |
| packages/core/test/repl/native-functions/debug-repl-fn.spec.ts | packages/core/repl/native-functions/debug-repl-fn.ts |
| packages/core/test/repl/native-functions/get-repl-fn.spec.ts | packages/core/repl/native-functions/get-repl-fn.ts |
| packages/core/test/repl/native-functions/help-repl-fn.spec.ts | packages/core/repl/native-functions/help-repl-fn.ts |
| packages/core/test/repl/native-functions/methods-repl-fn.spec.ts | packages/core/repl/native-functions/methods-repl-fn.ts |
| packages/core/test/repl/native-functions/resolve-repl-fn.spec.ts | packages/core/repl/native-functions/resolve-repl-fn.ts |
| packages/core/test/repl/native-functions/select-repl-fn.spec.ts | packages/core/repl/native-functions/select-relp-fn.ts |
| packages/core/test/repl/repl-context.spec.ts | packages/core/repl/repl-context.ts |
| packages/core/test/router/paths-explorer.spec.ts | packages/core/router/paths-explorer.ts |
| packages/core/test/router/route-params-factory.spec.ts | packages/core/router/route-params-factory.ts |
| packages/core/test/router/route-path-factory.spec.ts | packages/core/router/route-path-factory.ts |
| packages/core/test/router/router-exception-filters.spec.ts | packages/core/router/router-exception-filters.ts |
| packages/core/test/router/router-execution-context.spec.ts | packages/core/router/router-execution-context.ts |
| packages/core/test/router/router-explorer.spec.ts | packages/core/router/router-explorer.ts |
| packages/core/test/router/router-module.spec.ts | packages/core/router/router-module.ts |
| packages/core/test/router/router-proxy.spec.ts | packages/core/helpers/execution-context-host.ts |
| packages/core/test/router/router-proxy.spec.ts | packages/core/router/router-proxy.ts |
| packages/core/test/router/router-response-controller.spec.ts | packages/core/router/router-response-controller.ts |
| packages/core/test/router/routes-resolver.spec.ts | packages/core/router/routes-resolver.ts |
| packages/core/test/router/sse-stream.spec.ts | packages/core/router/sse-stream.ts |
| packages/core/test/router/utils/flat-routes.spec.ts | packages/core/router/utils/flatten-route-paths.util.ts |
| packages/core/test/scanner.spec.ts | packages/common/decorators/core/controller.decorator.ts |
| packages/core/test/scanner.spec.ts | packages/common/decorators/modules/module.decorator.ts |
| packages/core/test/scanner.spec.ts | packages/core/injector/instance-wrapper.ts |
| packages/core/test/scanner.spec.ts | packages/core/scanner.ts |
| packages/core/test/services/reflector.service.spec.ts | packages/core/services/reflector.service.ts |

## False Positives

| Test File | Production File |
|-----------|-----------------|
| packages/common/test/decorators/route-params.decorator.spec.ts | packages/common/pipes/parse-int.pipe.ts |
| packages/core/test/injector/internal-core-module/internal-core-module-factory.spec.ts | packages/core/injector/lazy-module-loader/lazy-module-loader.ts |
| packages/core/test/injector/internal-core-module/internal-core-module-factory.spec.ts | packages/core/injector/modules-container.ts |
| packages/core/test/injector/lazy-module-loader/lazy-module-loader.spec.ts | packages/core/injector/modules-container.ts |
| packages/core/test/router/router-execution-context.spec.ts | packages/common/decorators/http/route-params.decorator.ts |
| packages/core/test/router/routes-resolver.spec.ts | packages/core/injector/container.ts |

## False Negatives

| Test File | Production File | Evidence |
|-----------|-----------------|----------|
| packages/common/test/exceptions/http.exception.spec.ts | packages/common/exceptions/bad-request.exception.ts | barrel_import, constructor_usage, symbol_assertion |
| packages/common/test/exceptions/http.exception.spec.ts | packages/common/exceptions/http.exception.ts | barrel_import, constructor_usage, filename_match, symbol_assertion, test_name_match |
| packages/core/test/injector/helpers/provider-classifier.spec.ts | packages/common/interfaces/modules/provider.interface.ts | barrel_import, call_usage, symbol_assertion, test_name_match |
| packages/core/test/injector/helpers/silent-logger.spec.ts | packages/common/services/logger.service.ts | barrel_import, call_usage, symbol_assertion, test_name_match |
| packages/core/test/injector/instance-wrapper.spec.ts | packages/common/interfaces/scope-options.interface.ts | barrel_import, call_usage, symbol_assertion, test_name_match |
| packages/core/test/inspector/serialized-graph.spec.ts | packages/core/inspector/interfaces/edge.interface.ts | call_usage, direct_import, test_name_match |
| packages/core/test/inspector/serialized-graph.spec.ts | packages/core/inspector/interfaces/node.interface.ts | call_usage, direct_import, symbol_assertion, test_name_match |
| packages/core/test/pipes/params-token-factory.spec.ts | packages/common/enums/route-paramtypes.enum.ts | direct_import, symbol_assertion, test_name_match |
| packages/core/test/router/route-params-factory.spec.ts | packages/common/enums/route-paramtypes.enum.ts | direct_import, symbol_assertion, test_name_match |
| packages/core/test/router/router-response-controller.spec.ts | packages/common/enums/request-method.enum.ts | barrel_import, symbol_assertion, test_name_match |
| packages/core/test/scanner.spec.ts | packages/common/decorators/core/injectable.decorator.ts | barrel_import, call_usage, symbol_assertion, test_name_match |

