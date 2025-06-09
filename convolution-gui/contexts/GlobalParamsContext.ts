import { Dispatch, SetStateAction } from 'react';
import { createContext } from 'react';

import { PluginParams } from '../../bindings/PluginParams';

export const GlobalParametersContext = createContext<{
	parameters: PluginParams;
	setParameters: Dispatch<SetStateAction<PluginParams>>;
} | null>(null);
