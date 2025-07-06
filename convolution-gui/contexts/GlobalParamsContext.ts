import { GlobalParameters } from '@/lib/parameters';
import { Dispatch, SetStateAction } from 'react';
import { createContext } from 'react';

export const GlobalParametersContext = createContext<{
	parameters: GlobalParameters;
	setParameters: Dispatch<SetStateAction<GlobalParameters>>;
} | null>(null);
