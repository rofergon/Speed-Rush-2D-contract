module github.com/burnt-labs/xion

go 1.22.7

replace (
	cosmossdk.io/core => cosmossdk.io/core v0.11.1
	github.com/99designs/keyring => github.com/cosmos/keyring v1.2.0
	github.com/dgrijalva/jwt-go => github.com/golang-jwt/jwt/v4 v4.4.2
	github.com/gin-gonic/gin => github.com/gin-gonic/gin v1.9.1
	github.com/gogo/protobuf => github.com/regen-network/protobuf v1.3.3-alpha.regen.1
	github.com/larry0x/abstract-account => github.com/burnt-labs/abstract-account v0.0.0-20241017182826-e40d245c944a
	github.com/strangelove-ventures/tokenfactory => github.com/strangelove-ventures/tokenfactory v0.50.3-wasmvm2
	github.com/syndtr/goleveldb => github.com/syndtr/goleveldb v1.0.1-0.20210819022825-2ae1ddf74ef7
	github.com/vektra/mockery/v2 => github.com/vektra/mockery/v2 v2.14.0
	golang.org/x/crypto => golang.org/x/crypto v0.21.0
	golang.org/x/net => golang.org/x/net v0.22.0
)

require (
	cosmossdk.io/api v0.7.6
	cosmossdk.io/client/v2 v2.0.0-beta.4
	cosmossdk.io/collections v0.4.0
	cosmossdk.io/core v0.12.0
	cosmossdk.io/depinject v1.1.0
	cosmossdk.io/errors v1.0.1
	cosmossdk.io/math v1.4.0
	cosmossdk.io/store v1.1.1
	cosmossdk.io/tools/confix v0.1.2
	cosmossdk.io/x/evidence v0.1.1
	cosmossdk.io/x/feegrant v0.1.1
	cosmossdk.io/x/nft v0.1.1
	cosmossdk.io/x/tx v0.13.7
	cosmossdk.io/x/upgrade v0.1.4
	github.com/CosmWasm/wasmd v0.53.0
	github.com/CosmWasm/wasmvm/v2 v2.1.2
	github.com/cometbft/cometbft v0.38.12
	github.com/cosmos/cosmos-db v1.1.0
	github.com/cosmos/cosmos-proto v1.0.0-beta.5
	github.com/cosmos/cosmos-sdk v0.50.11
	github.com/cosmos/gogoproto v1.7.0
	github.com/cosmos/ibc-apps/middleware/packet-forward-middleware/v8 v8.0.2
	github.com/cosmos/ibc-apps/modules/ibc-hooks/v8 v8.0.0-20240530162148-4827cf263165
	github.com/cosmos/ibc-go/modules/capability v1.0.1
	github.com/cosmos/ibc-go/modules/light-clients/08-wasm v0.4.2-0.20240730185033-ccd4dc278e72
	github.com/cosmos/ibc-go/v8 v8.5.1
	github.com/cosmos/rosetta v0.50.6
	github.com/dvsekhvalnov/jose2go v1.6.0
	github.com/go-webauthn/webauthn v0.9.1
	github.com/golang/protobuf v1.5.4
	github.com/gorilla/mux v1.8.1
	github.com/grpc-ecosystem/grpc-gateway v1.16.0
	github.com/hashicorp/go-metrics v0.5.3
	github.com/larry0x/abstract-account v0.0.0-20240202022305-255071ed91ee
	github.com/lestrrat-go/jwx/v2 v2.0.21
	github.com/osmosis-labs/fee-abstraction/v8 v8.0.1
	github.com/prometheus/client_golang v1.20.1
	github.com/spf13/cast v1.7.0
	github.com/spf13/cobra v1.8.1
	github.com/spf13/viper v1.19.0
	github.com/strangelove-ventures/tokenfactory v0.50.3
	github.com/stretchr/testify v1.9.0
	github.com/syndtr/goleveldb v1.0.1-0.20220721030215-126854af5e6d
	go.uber.org/mock v0.4.0
	google.golang.org/genproto/googleapis/api v0.0.0-20240814211410-ddb44dafa142
	google.golang.org/grpc v1.67.1
	gotest.tools/v3 v3.5.1
	sigs.k8s.io/yaml v1.4.0
)

require (
	cloud.google.com/go/auth v0.6.0 // indirect
	cloud.google.com/go/auth/oauth2adapt v0.2.2 // indirect
	github.com/DataDog/datadog-go v3.2.0+incompatible // indirect
	github.com/DataDog/zstd v1.5.5 // indirect
	github.com/PuerkitoBio/purell v1.1.1 // indirect
	github.com/PuerkitoBio/urlesc v0.0.0-20170810143723-de5bf2ad4578 // indirect
	github.com/bits-and-blooms/bitset v1.10.0 // indirect
	github.com/cockroachdb/datadriven v1.0.3-0.20230801171734-e384cf455877 // indirect
	github.com/cockroachdb/fifo v0.0.0-20240606204812-0bbfbd93a7ce // indirect
	github.com/cockroachdb/pebble v1.1.2 // indirect
	github.com/cockroachdb/tokenbucket v0.0.0-20230807174530-cc333fc44b06 // indirect
	github.com/cometbft/cometbft-db v0.12.0 // indirect
	github.com/creachadair/atomicfile v0.3.1 // indirect
	github.com/creachadair/tomledit v0.0.24 // indirect
	github.com/dgraph-io/badger/v4 v4.2.0 // indirect
	github.com/distribution/reference v0.5.0 // indirect
	github.com/docker/go-connections v0.5.0 // indirect
	github.com/emicklei/dot v1.6.2 // indirect
	github.com/fatih/color v1.15.0 // indirect
	github.com/gogo/status v1.1.0 // indirect
	github.com/golang/mock v1.6.0 // indirect
	github.com/google/flatbuffers v1.12.1 // indirect
	github.com/goware/urlx v0.3.2 // indirect
	github.com/hashicorp/go-hclog v1.5.0 // indirect
	github.com/hashicorp/go-plugin v1.5.2 // indirect
	github.com/hashicorp/go-uuid v1.0.3 // indirect
	github.com/hashicorp/golang-lru/v2 v2.0.7 // indirect
	github.com/hashicorp/yamux v0.1.1 // indirect
	github.com/iancoleman/strcase v0.3.0 // indirect
	github.com/munnerz/goautoneg v0.0.0-20191010083416-a7dc8b61c822 // indirect
	github.com/oasisprotocol/curve25519-voi v0.0.0-20230904125328-1f23a7beb09a // indirect
	github.com/oklog/run v1.1.0 // indirect
	github.com/onsi/gomega v1.27.10 // indirect
	github.com/sagikazarmark/locafero v0.4.0 // indirect
	github.com/sagikazarmark/slog-shim v0.1.0 // indirect
	github.com/shamaton/msgpack/v2 v2.2.0 // indirect
	github.com/sourcegraph/conc v0.3.0 // indirect
	go.opentelemetry.io/contrib/instrumentation/google.golang.org/grpc/otelgrpc v0.49.0 // indirect
	go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp v0.49.0 // indirect
	go.uber.org/multierr v1.11.0 // indirect
	golang.org/x/time v0.5.0 // indirect
)

require (
	cloud.google.com/go v0.115.0 // indirect
	cloud.google.com/go/compute/metadata v0.5.0 // indirect
	cloud.google.com/go/iam v1.1.9 // indirect
	cloud.google.com/go/storage v1.41.0 // indirect
	cosmossdk.io/log v1.4.1
	filippo.io/edwards25519 v1.1.0 // indirect
	github.com/99designs/go-keychain v0.0.0-20191008050251-8e49817e8af4 // indirect
	github.com/99designs/keyring v1.2.2 // indirect
	github.com/aws/aws-sdk-go v1.44.224 // indirect
	github.com/beorn7/perks v1.0.1 // indirect
	github.com/bgentry/go-netrc v0.0.0-20140422174119-9fd32a8b3d3d // indirect
	github.com/bgentry/speakeasy v0.1.1-0.20220910012023-760eaf8b6816 // indirect
	github.com/btcsuite/btcd/btcec/v2 v2.3.4 // indirect
	github.com/cenkalti/backoff/v4 v4.2.1 // indirect
	github.com/cespare/xxhash/v2 v2.3.0 // indirect
	github.com/chzyer/readline v1.5.1 // indirect
	github.com/cockroachdb/apd/v2 v2.0.2 // indirect
	github.com/cockroachdb/errors v1.11.3 // indirect
	github.com/cockroachdb/logtags v0.0.0-20230118201751-21c54148d20b // indirect
	github.com/cockroachdb/redact v1.1.5 // indirect
	github.com/coinbase/rosetta-sdk-go/types v1.0.0 // indirect
	github.com/cosmos/btcutil v1.0.5 // indirect
	github.com/cosmos/go-bip39 v1.0.0 // indirect
	github.com/cosmos/gogogateway v1.2.0 // indirect
	github.com/cosmos/iavl v1.2.2 // indirect
	github.com/cosmos/ics23/go v0.11.0 // indirect
	github.com/cosmos/ledger-cosmos-go v0.13.3 // indirect
	github.com/cosmos/rosetta-sdk-go v0.10.0 // indirect
	github.com/danieljoos/wincred v1.1.2 // indirect
	github.com/davecgh/go-spew v1.1.2-0.20180830191138-d8f796af33cc // indirect
	github.com/decred/dcrd/dcrec/secp256k1/v4 v4.2.0 // indirect
	github.com/desertbit/timer v0.0.0-20180107155436-c41aec40b27f // indirect
	github.com/dgraph-io/ristretto v0.1.1 // indirect
	github.com/dustin/go-humanize v1.0.1 // indirect
	github.com/felixge/httpsnoop v1.0.4 // indirect
	github.com/fsnotify/fsnotify v1.7.0 // indirect
	github.com/fxamacker/cbor/v2 v2.5.0 // indirect
	github.com/getsentry/sentry-go v0.27.0 // indirect
	github.com/go-kit/kit v0.12.0 // indirect
	github.com/go-kit/log v0.2.1 // indirect
	github.com/go-logfmt/logfmt v0.6.0 // indirect
	github.com/go-logr/logr v1.4.1 // indirect
	github.com/go-logr/stdr v1.2.2 // indirect
	github.com/go-webauthn/x v0.1.4 // indirect
	github.com/goccy/go-json v0.10.2 // indirect
	github.com/godbus/dbus v0.0.0-20190726142602-4481cbc300e2 // indirect
	github.com/gogo/googleapis v1.4.1 // indirect
	github.com/gogo/protobuf v1.3.3 // indirect
	github.com/golang-jwt/jwt/v5 v5.1.0
	github.com/golang/glog v1.2.2 // indirect
	github.com/golang/groupcache v0.0.0-20210331224755-41bb18bfe9da // indirect
	github.com/golang/snappy v0.0.5-0.20220116011046-fa5810519dcb // indirect
	github.com/google/btree v1.1.3 // indirect
	github.com/google/go-cmp v0.6.0 // indirect
	github.com/google/go-tpm v0.9.0 // indirect
	github.com/google/gofuzz v1.2.0 // indirect
	github.com/google/orderedcode v0.0.1 // indirect
	github.com/google/s2a-go v0.1.7 // indirect
	github.com/google/uuid v1.6.0 // indirect
	github.com/googleapis/enterprise-certificate-proxy v0.3.2 // indirect
	github.com/googleapis/gax-go/v2 v2.12.5 // indirect
	github.com/gorilla/handlers v1.5.2 // indirect
	github.com/gorilla/websocket v1.5.3 // indirect
	github.com/grpc-ecosystem/go-grpc-middleware v1.4.0 // indirect
	github.com/gsterjov/go-libsecret v0.0.0-20161001094733-a6f4afe4910c // indirect
	github.com/hashicorp/go-cleanhttp v0.5.2 // indirect
	github.com/hashicorp/go-getter v1.7.5 // indirect
	github.com/hashicorp/go-immutable-radix v1.3.1 // indirect
	github.com/hashicorp/go-safetemp v1.0.0 // indirect
	github.com/hashicorp/go-version v1.6.0 // indirect
	github.com/hashicorp/golang-lru v1.0.2 // indirect
	github.com/hashicorp/hcl v1.0.0 // indirect
	github.com/hdevalence/ed25519consensus v0.1.0 // indirect
	github.com/huandu/skiplist v1.2.0 // indirect
	github.com/iancoleman/orderedmap v0.3.0 // indirect
	github.com/improbable-eng/grpc-web v0.15.0 // indirect
	github.com/inconshreveable/mousetrap v1.1.0 // indirect
	github.com/jmespath/go-jmespath v0.4.0 // indirect
	github.com/jmhodges/levigo v1.0.0 // indirect
	github.com/klauspost/compress v1.17.9 // indirect
	github.com/kr/pretty v0.3.1 // indirect
	github.com/kr/text v0.2.0 // indirect
	github.com/lestrrat-go/backoff/v2 v2.0.8 // indirect
	github.com/lestrrat-go/blackmagic v1.0.2 // indirect
	github.com/lestrrat-go/httpcc v1.0.1 // indirect
	github.com/lestrrat-go/httprc v1.0.5 // indirect
	github.com/lestrrat-go/iter v1.0.2 // indirect
	github.com/lestrrat-go/jwx v1.2.28
	github.com/lestrrat-go/option v1.0.1 // indirect
	github.com/lib/pq v1.10.9 // indirect
	github.com/linxGnu/grocksdb v1.8.14 // indirect
	github.com/magiconair/properties v1.8.7 // indirect
	github.com/manifoldco/promptui v0.9.0 // indirect
	github.com/mattn/go-colorable v0.1.13 // indirect
	github.com/mattn/go-isatty v0.0.20 // indirect
	github.com/minio/highwayhash v1.0.2 // indirect
	github.com/mitchellh/go-homedir v1.1.0 // indirect
	github.com/mitchellh/go-testing-interface v1.14.1 // indirect
	github.com/mitchellh/mapstructure v1.5.0 // indirect
	github.com/mtibben/percent v0.2.1 // indirect
	github.com/opencontainers/go-digest v1.0.0 // indirect
	github.com/pelletier/go-toml/v2 v2.2.2 // indirect
	github.com/petermattis/goid v0.0.0-20231207134359-e60b3f734c67 // indirect
	github.com/pkg/errors v0.9.1 // indirect
	github.com/pmezard/go-difflib v1.0.1-0.20181226105442-5d4384ee4fb2 // indirect
	github.com/prometheus/client_model v0.6.1 // indirect
	github.com/prometheus/common v0.55.0 // indirect
	github.com/prometheus/procfs v0.15.1 // indirect
	github.com/rcrowley/go-metrics v0.0.0-20201227073835-cf1acfcdf475 // indirect
	github.com/rogpeppe/go-internal v1.12.0 // indirect
	github.com/rs/cors v1.11.1 // indirect
	github.com/rs/zerolog v1.33.0 // indirect
	github.com/sasha-s/go-deadlock v0.3.1 // indirect
	github.com/segmentio/asm v1.2.0 // indirect
	github.com/spf13/afero v1.11.0 // indirect
	github.com/spf13/pflag v1.0.5 // indirect
	github.com/subosito/gotenv v1.6.0 // indirect
	github.com/tendermint/go-amino v0.16.0 // indirect
	github.com/tidwall/btree v1.7.0 // indirect
	github.com/ulikunitz/xz v0.5.11 // indirect
	github.com/x448/float16 v0.8.4 // indirect
	github.com/zondax/hid v0.9.2 // indirect
	github.com/zondax/ledger-go v0.14.3 // indirect
	go.etcd.io/bbolt v1.4.0-alpha.0.0.20240404170359-43604f3112c5 // indirect
	go.opencensus.io v0.24.0 // indirect
	go.opentelemetry.io/otel v1.24.0 // indirect
	go.opentelemetry.io/otel/metric v1.24.0 // indirect
	go.opentelemetry.io/otel/trace v1.24.0 // indirect
	golang.org/x/crypto v0.27.0 // indirect
	golang.org/x/exp v0.0.0-20240613232115-7f521ea00fb8 // indirect
	golang.org/x/net v0.29.0 // indirect
	golang.org/x/oauth2 v0.22.0 // indirect
	golang.org/x/sync v0.8.0 // indirect
	golang.org/x/sys v0.25.0 // indirect
	golang.org/x/term v0.24.0 // indirect
	golang.org/x/text v0.18.0 // indirect
	google.golang.org/api v0.186.0 // indirect
	google.golang.org/genproto v0.0.0-20240701130421-f6361c86f094 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20240930140551-af27646dc61f // indirect
	google.golang.org/protobuf v1.35.1
	gopkg.in/ini.v1 v1.67.0 // indirect
	gopkg.in/yaml.v2 v2.4.0 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
	nhooyr.io/websocket v1.8.10 // indirect
	pgregory.net/rapid v1.1.0 // indirect
)
