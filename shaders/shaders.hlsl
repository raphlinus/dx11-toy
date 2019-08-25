float4 vs_main( float4 pos : POSITION ) : SV_POSITION
{
	return pos;
}

float4 ps_main() : SV_TARGET
{
	return float4(1.0f, 1.0f, 1.0f, 1.0f);
}
