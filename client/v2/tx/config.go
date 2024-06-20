package tx

import (
	"errors"

	"google.golang.org/protobuf/reflect/protoreflect"

	apitxsigning "cosmossdk.io/api/cosmos/tx/signing/v1beta1"
	"cosmossdk.io/core/address"
	txdecode "cosmossdk.io/x/tx/decode"
	"cosmossdk.io/x/tx/signing"
	"cosmossdk.io/x/tx/signing/aminojson"
	"cosmossdk.io/x/tx/signing/direct"
	"cosmossdk.io/x/tx/signing/directaux"
	"cosmossdk.io/x/tx/signing/textual"

	"github.com/cosmos/cosmos-sdk/codec"
)

var (
	_ TxConfig         = txConfig{}
	_ TxEncodingConfig = defaultEncodingConfig{}
	_ TxSigningConfig  = defaultTxSigningConfig{}

	defaultEnabledSignModes = []apitxsigning.SignMode{
		apitxsigning.SignMode_SIGN_MODE_DIRECT,
		apitxsigning.SignMode_SIGN_MODE_DIRECT_AUX,
		apitxsigning.SignMode_SIGN_MODE_LEGACY_AMINO_JSON,
	}
)

// TxConfig is an interface that a client can use to generate a concrete transaction type
// defined by the application. The returned type must implement the TxBuilder interface.
type TxConfig interface {
	TxEncodingConfig
	TxSigningConfig
	TxBuilderProvider
}

// TxEncodingConfig defines the interface for transaction encoding and decoding.
// It provides methods for both binary and JSON encoding/decoding.
type TxEncodingConfig interface {
	// TxEncoder returns an encoder for binary transaction encoding.
	TxEncoder() txApiEncoder
	// TxDecoder returns a decoder for binary transaction decoding.
	TxDecoder() txApiDecoder
	// TxJSONEncoder returns an encoder for JSON transaction encoding.
	TxJSONEncoder() txApiEncoder
	// TxJSONDecoder returns a decoder for JSON transaction decoding.
	TxJSONDecoder() txApiDecoder
}

// TxSigningConfig defines the interface for transaction signing configurations.
type TxSigningConfig interface {
	// SignModeHandler returns a reference to the HandlerMap which manages the different signing modes.
	SignModeHandler() *signing.HandlerMap
	// SigningContext returns a reference to the Context which holds additional data required during signing.
	SigningContext() *signing.Context
	// MarshalSignatureJSON takes a slice of Signature objects and returns their JSON encoding.
	MarshalSignatureJSON([]Signature) ([]byte, error)
	// UnmarshalSignatureJSON takes a JSON byte slice and returns a slice of Signature objects.
	UnmarshalSignatureJSON([]byte) ([]Signature, error)
}

// ConfigOptions defines the configuration options for transaction processing.
type ConfigOptions struct {
	AddressCodec address.Codec
	Decoder      Decoder
	Cdc          codec.BinaryCodec

	ValidatorAddressCodec address.Codec
	FileResolver          signing.ProtoFileResolver
	TypeResolver          signing.TypeResolver
	CustomGetSigner       map[protoreflect.FullName]signing.GetSignersFunc
	MaxRecursionDepth     int

	EnablesSignModes           []apitxsigning.SignMode
	CustomSignModes            []signing.SignModeHandler
	TextualCoinMetadataQueryFn textual.CoinMetadataQueryFn
}

// validate checks the ConfigOptions for required fields and sets default values where necessary.
// It returns an error if any required field is missing.
func (c *ConfigOptions) validate() error {
	if c.AddressCodec == nil {
		return errors.New("address codec cannot be nil")
	}
	if c.Cdc == nil {
		return errors.New("codec cannot be nil")
	}
	if c.ValidatorAddressCodec == nil {
		return errors.New("validator address codec cannot be nil")
	}

	// set default signModes if none are provided
	if len(c.EnablesSignModes) == 0 {
		c.EnablesSignModes = defaultEnabledSignModes
	}
	return nil
}

// txConfig is a struct that embeds TxBuilderProvider, TxEncodingConfig, and TxSigningConfig interfaces.
type txConfig struct {
	TxBuilderProvider
	TxEncodingConfig
	TxSigningConfig
}

// NewTxConfig creates a new TxConfig instance using the provided ConfigOptions.
// It validates the options, initializes the signing context, and sets up the decoder if not provided.
func NewTxConfig(options ConfigOptions) (TxConfig, error) {
	err := options.validate()
	if err != nil {
		return nil, err
	}

	signingCtx, err := newDefaultTxSigningConfig(options)
	if err != nil {
		return nil, err
	}

	if options.Decoder == nil {
		options.Decoder, err = txdecode.NewDecoder(txdecode.Options{
			SigningContext: signingCtx.SigningContext(),
			ProtoCodec:     options.Cdc,
		})
		if err != nil {
			return nil, err
		}
	}

	return &txConfig{
		TxBuilderProvider: NewBuilderProvider(options.AddressCodec, options.Decoder, options.Cdc),
		TxEncodingConfig:  defaultEncodingConfig{},
		TxSigningConfig:   signingCtx,
	}, nil
}

// defaultEncodingConfig is an empty struct that implements the TxEncodingConfig interface.
type defaultEncodingConfig struct{}

// TxEncoder returns the default transaction encoder.
func (t defaultEncodingConfig) TxEncoder() txApiEncoder {
	return txEncoder
}

// TxDecoder returns the default transaction decoder.
func (t defaultEncodingConfig) TxDecoder() txApiDecoder {
	return txDecoder
}

// TxJSONEncoder returns the default JSON transaction encoder.
func (t defaultEncodingConfig) TxJSONEncoder() txApiEncoder {
	return txJsonEncoder
}

// TxJSONDecoder returns the default JSON transaction decoder.
func (t defaultEncodingConfig) TxJSONDecoder() txApiDecoder {
	return txJsonDecoder
}

// defaultTxSigningConfig is a struct that holds the signing context and handler map.
type defaultTxSigningConfig struct {
	signingCtx *signing.Context
	handlerMap *signing.HandlerMap
}

// newDefaultTxSigningConfig creates a new defaultTxSigningConfig instance using the provided ConfigOptions.
// It initializes the signing context and handler map.
func newDefaultTxSigningConfig(opts ConfigOptions) (*defaultTxSigningConfig, error) {
	signingCtx, err := newSigningContext(opts)
	if err != nil {
		return nil, err
	}

	handlerMap, err := newHandlerMap(opts, signingCtx)
	if err != nil {
		return nil, err
	}

	return &defaultTxSigningConfig{
		signingCtx: signingCtx,
		handlerMap: handlerMap,
	}, nil
}

// SignModeHandler returns the handler map that manages the different signing modes.
func (t defaultTxSigningConfig) SignModeHandler() *signing.HandlerMap {
	return t.handlerMap
}

// SigningContext returns the signing context that holds additional data required during signing.
func (t defaultTxSigningConfig) SigningContext() *signing.Context {
	return t.signingCtx
}

// MarshalSignatureJSON takes a slice of Signature objects and returns their JSON encoding.
// This method is not yet implemented and will panic if called.
func (t defaultTxSigningConfig) MarshalSignatureJSON(signatures []Signature) ([]byte, error) {
	// TODO implement me
	panic("implement me")
}

// UnmarshalSignatureJSON takes a JSON byte slice and returns a slice of Signature objects.
// This method is not yet implemented and will panic if called.
func (t defaultTxSigningConfig) UnmarshalSignatureJSON(bytes []byte) ([]Signature, error) {
	// TODO implement me
	panic("implement me")
}

// newSigningContext creates a new signing context using the provided ConfigOptions.
// Returns a signing.Context instance or an error if initialization fails.
func newSigningContext(opts ConfigOptions) (*signing.Context, error) {
	return signing.NewContext(signing.Options{
		FileResolver:          opts.FileResolver,
		TypeResolver:          opts.TypeResolver,
		AddressCodec:          opts.AddressCodec,
		ValidatorAddressCodec: opts.ValidatorAddressCodec,
		CustomGetSigners:      opts.CustomGetSigner,
		MaxRecursionDepth:     opts.MaxRecursionDepth,
	})
}

// newHandlerMap constructs a new HandlerMap based on the provided ConfigOptions and signing context.
// It initializes handlers for each enabled and custom sign mode specified in the options.
func newHandlerMap(opts ConfigOptions, signingCtx *signing.Context) (*signing.HandlerMap, error) {
	lenSignModes := len(opts.EnablesSignModes)
	handlers := make([]signing.SignModeHandler, lenSignModes+len(opts.CustomSignModes))

	for i, m := range opts.EnablesSignModes {
		var err error
		switch m {
		case apitxsigning.SignMode_SIGN_MODE_DIRECT:
			handlers[i] = &direct.SignModeHandler{}
		case apitxsigning.SignMode_SIGN_MODE_TEXTUAL:
			if opts.TextualCoinMetadataQueryFn == nil {
				return nil, errors.New("cannot enable SIGN_MODE_TEXTUAL without a TextualCoinMetadataQueryFn")
			}
			handlers[i], err = textual.NewSignModeHandler(textual.SignModeOptions{
				CoinMetadataQuerier: opts.TextualCoinMetadataQueryFn,
				FileResolver:        signingCtx.FileResolver(),
				TypeResolver:        signingCtx.TypeResolver(),
			})
			if err != nil {
				return nil, err
			}
		case apitxsigning.SignMode_SIGN_MODE_DIRECT_AUX:
			handlers[i], err = directaux.NewSignModeHandler(directaux.SignModeHandlerOptions{
				TypeResolver:   signingCtx.TypeResolver(),
				SignersContext: signingCtx,
			})
			if err != nil {
				return nil, err
			}
		case apitxsigning.SignMode_SIGN_MODE_LEGACY_AMINO_JSON:
			handlers[i] = aminojson.NewSignModeHandler(aminojson.SignModeHandlerOptions{
				FileResolver: signingCtx.FileResolver(),
				TypeResolver: opts.TypeResolver,
			})
		}
	}
	for i, m := range opts.CustomSignModes {
		handlers[i+lenSignModes] = m
	}

	handler := signing.NewHandlerMap(handlers...)
	return handler, nil
}
